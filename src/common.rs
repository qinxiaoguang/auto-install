// 统一错误
pub type Error = failure::Error;
//pub type Error = Box<dyn std::error::Error>;

pub type Result<T> = std::result::Result<T, Error>;
