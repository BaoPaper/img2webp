use clap::Parser;

/// 将图片文件转换为 WebP 格式
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// 输入文件或文件夹路径
    pub input: String,

    /// 输出文件或文件夹路径
    #[arg(short, long)]
    pub output: Option<String>,

    /// 转换质量 (0-100)
    #[arg(short, long, default_value_t = 80, value_parser = clap::value_parser!(u8).range(0..=100))]
    pub quality: u8,

    /// 递归处理子文件夹
    #[arg(short, long)]
    pub recursive: bool,

    /// 并发处理数
    #[arg(short, long, default_value_t = 4)]
    pub concurrent: usize,

    /// 替换原文件
    #[arg(long)]
    pub replace: bool,
}