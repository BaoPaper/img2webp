use anyhow::{Result, anyhow};
use std::path::{Path, PathBuf};
use std::process::Command;
use tokio::process::Command as TokioCommand;

/// 转换器结构体
pub struct Converter {
    quality: u8,
}

impl Converter {
    /// 创建新的转换器实例
    pub fn new(quality: u8) -> Self {
        Self { quality }
    }

    /// 转换单个文件
    pub fn convert_file(&self, input: &Path, output: &Path) -> Result<()> {
        let output_result = Command::new("ffmpeg")
            .arg("-i")
            .arg(input)
            .arg("-quality")
            .arg(self.quality.to_string())
            .arg("-y") // 覆盖输出文件
            .arg(output)
            .output()?;

        if !output_result.status.success() {
            let stderr = String::from_utf8_lossy(&output_result.stderr);
            return Err(anyhow!("FFmpeg conversion failed: {}", stderr));
        }

        Ok(())
    }

    /// 并发转换多个文件
    pub async fn convert_files_parallel(
        &self,
        files: Vec<(PathBuf, PathBuf)>,
        concurrent: usize,
        progress: &crate::progress::ProgressTracker,
    ) -> Result<Vec<Result<(), anyhow::Error>>> {
        // 限制并发数量
        let semaphore = std::sync::Arc::new(tokio::sync::Semaphore::new(concurrent));
        
        // 创建任务列表
        let mut handles = vec![];
        
        for (input, output) in files {
            let semaphore = semaphore.clone();
            let quality = self.quality;
            let progress = progress.clone();

            let handle = tokio::spawn(async move {
                // 获取信号量许可
                let _permit = semaphore.acquire().await.unwrap();
                
                let result = TokioCommand::new("ffmpeg")
                    .arg("-i")
                    .arg(&input)
                    .arg("-quality")
                    .arg(quality.to_string())
                    .arg("-y") // 覆盖输出文件
                    .arg(&output)
                    .output()
                    .await;
                
                match result {
                    Ok(output_result) => {
                        if output_result.status.success() {
                            progress.inc(1);
                            Ok(())
                        } else {
                            let stderr = String::from_utf8_lossy(&output_result.stderr);
                            Err(anyhow!("FFmpeg conversion failed for {:?}: {}", input, stderr))
                        }
                    }
                    Err(e) => {
                        Err(anyhow!("Failed to execute FFmpeg for {:?}: {}", input, e))
                    }
                }
            });
            
            handles.push(handle);
        }
        
        // 等待所有任务完成并收集结果
        let mut results = vec![];
        for handle in handles {
            let result = handle.await?;
            results.push(result);
        }
        
        Ok(results)
    }
}