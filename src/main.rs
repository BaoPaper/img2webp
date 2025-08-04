use anyhow::Result;
use clap::Parser;
use std::path::{Path, PathBuf};
use std::fs;

mod cli;
mod converter;
mod scanner;
mod progress;
mod error;

#[tokio::main]
async fn main() -> Result<()> {
    // 解析命令行参数
    let args = cli::Args::parse();
    
    // 根据输入类型执行相应操作
    let result = if Path::new(&args.input).is_file() {
        handle_single_file(&args).await
    } else {
        handle_directory(&args).await
    };
    
    // 处理结果
    match result {
        Ok(_) => {
            println!("Conversion completed successfully!");
        }
        Err(e) => {
            error::handle_error(&e);
            std::process::exit(1);
        }
    }
    
    Ok(())
}

/// 处理单文件转换
async fn handle_single_file(args: &cli::Args) -> Result<()> {
    let input_path = Path::new(&args.input);
    
    // 检查输入文件是否存在
    if !input_path.exists() {
        return Err(anyhow::anyhow!("Input file does not exist: {:?}", input_path));
    }
    
    // 检查是否为支持的图片格式
    if !scanner::is_image_file(input_path) {
        return Err(anyhow::anyhow!("Input file is not a supported image format: {:?}", input_path));
    }
    
    // 确定输出路径
    let output_path = if let Some(output) = &args.output {
        PathBuf::from(output)
    } else if args.replace {
        // 替换原文件，更改扩展名为 .webp
        let mut new_name = input_path.file_stem().unwrap().to_os_string();
        new_name.push(".webp");
        input_path.with_file_name(new_name)
    } else {
        // 在原文件同目录下创建同名 .webp 文件
        let mut new_name = input_path.file_stem().unwrap().to_os_string();
        new_name.push(".webp");
        input_path.with_file_name(new_name)
    };
    
    // 创建转换器
    let converter = converter::Converter::new(args.quality);
    
    // 执行转换
    println!("Converting {:?} to {:?}...", input_path, output_path);
    converter.convert_file(input_path, &output_path)?;
    
    // 如果需要替换原文件，则删除原文件
    if args.replace {
        fs::remove_file(input_path)?;
        println!("Original file deleted.");
    }
    
    println!("Conversion completed: {:?}", output_path);
    Ok(())
}

/// 处理目录转换
async fn handle_directory(args: &cli::Args) -> Result<()> {
    let input_path = Path::new(&args.input);
    
    // 检查输入目录是否存在
    if !input_path.exists() {
        return Err(anyhow::anyhow!("Input directory does not exist: {:?}", input_path));
    }
    
    if !input_path.is_dir() {
        return Err(anyhow::anyhow!("Input path is not a directory: {:?}", input_path));
    }
    
    // 扫描图片文件
    println!("Scanning for image files...");
    let image_files = scanner::scan_image_files(input_path, args.recursive)?;
    
    if image_files.is_empty() {
        println!("No image files found in the directory.");
        return Ok(());
    }
    
    println!("Found {} image files to convert.", image_files.len());
    
    // 构建转换任务列表
    let mut convert_tasks = Vec::new();
    for input_file in &image_files {  // 使用引用
        // 确定输出路径
        let output_file = if let Some(output) = &args.output {
            let output_dir = Path::new(output);
            // 确保输出目录存在
            fs::create_dir_all(output_dir)?;
            
            // 保持相对路径结构
            let relative_path = input_file.strip_prefix(input_path).unwrap_or(input_file);
            let mut new_name = relative_path.file_stem().unwrap().to_os_string();
            new_name.push(".webp");
            output_dir.join(relative_path.with_file_name(new_name))
        } else if args.replace {
            // 替换原文件，更改扩展名为 .webp
            let mut new_name = input_file.file_stem().unwrap().to_os_string();
            new_name.push(".webp");
            input_file.with_file_name(new_name)
        } else {
            // 在原文件同目录下创建同名 .webp 文件
            let mut new_name = input_file.file_stem().unwrap().to_os_string();
            new_name.push(".webp");
            input_file.with_file_name(new_name)
        };
        
        // 确保输出文件的目录存在
        if let Some(parent) = output_file.parent() {
            fs::create_dir_all(parent)?;
        }
        
        convert_tasks.push((input_file.clone(), output_file));  // 克隆路径
    }
    
    // 保存一份副本用于替换原文件
    let original_tasks = convert_tasks.clone();
    
    // 创建进度条
    let progress = progress::ProgressTracker::new(convert_tasks.len() as u64);
    
    // 创建转换器
    let converter = converter::Converter::new(args.quality);
    
    // 并发执行转换
    progress.set_message("Converting files...");
    let results = converter.convert_files_parallel(convert_tasks, args.concurrent).await?;
    
    // 更新进度
    let mut success_count = 0;
    let mut error_count = 0;
    let mut errors = Vec::new();
    
    for result in results {
        if result.is_ok() {
            success_count += 1;
        } else {
            error_count += 1;
            errors.push(result.unwrap_err());
        }
        progress.inc(1);
    }
    
    progress.finish();
    
    // 处理错误
    if !errors.is_empty() {
        println!("{} files converted successfully, {} files failed.", success_count, error_count);
        error::handle_errors(&errors);
    } else {
        println!("All {} files converted successfully!", success_count);
    }
    
    // 如果需要替换原文件，则删除原文件
    if args.replace {
        let mut deleted_count = 0;
        for (input_file, _) in &original_tasks {
            if fs::remove_file(input_file).is_ok() {
                deleted_count += 1;
            }
        }
        println!("{} original files deleted.", deleted_count);
    }
    
    Ok(())
}
