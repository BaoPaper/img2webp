use indicatif::{ProgressBar, ProgressStyle};

/// 进度跟踪器
#[derive(Clone)]
pub struct ProgressTracker {
    bar: ProgressBar,
}

impl ProgressTracker {
    /// 创建新的进度跟踪器
    pub fn new(total: u64) -> Self {
        let bar = ProgressBar::new(total);
        bar.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta}) {msg}")
                .unwrap()
                .progress_chars("#>-"),
        );
        Self { bar }
    }

    /// 设置进度消息
    pub fn set_message(&self, msg: &str) {
        self.bar.set_message(msg.to_string());
    }

    /// 增加进度
    pub fn inc(&self, delta: u64) {
        self.bar.inc(delta);
    }

    /// 完成进度显示
    pub fn finish(&self) {
        self.bar.finish();
    }

}