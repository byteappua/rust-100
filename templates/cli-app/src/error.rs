use thiserror::Error;

/// 应用错误类型
#[derive(Error, Debug)]
pub enum AppError {
    /// IO 错误
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// 配置错误
    #[error("Config error: {0}")]
    Config(String),

    /// 解析错误
    #[error("Parse error: {0}")]
    Parse(String),

    /// 其他错误
    #[error("Error: {0}")]
    Other(String),
}

/// Result 类型别名
pub type AppResult<T> = Result<T, AppError>;
