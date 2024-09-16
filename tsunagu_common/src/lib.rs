pub mod device;
pub mod discovery;
pub mod transfer;
pub mod encryption;
pub mod error;
pub mod models;
pub mod config;

pub use error::TsunaguError;
pub type Result<T> = std::result::Result<T, TsunaguError>;

pub fn common_function() -> &'static str {
    "This is a common function"
}
