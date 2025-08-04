use anyhow::Error;

/// 处理并显示错误信息
pub fn handle_error(error: &Error) {
    eprintln!("Error: {}", error);
    for cause in error.chain().skip(1) {
        eprintln!("Caused by: {}", cause);
    }
}

/// 处理并显示多个错误信息
pub fn handle_errors(errors: &[Error]) {
    for (i, error) in errors.iter().enumerate() {
        eprintln!("Error {}: {}", i + 1, error);
        for cause in error.chain().skip(1) {
            eprintln!("  Caused by: {}", cause);
        }
    }
}