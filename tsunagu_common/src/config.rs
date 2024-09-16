use std::path::PathBuf;

/// 应用配置
pub struct Config {
    pub device_name: String,
    pub save_directory: PathBuf,
    pub allow_auto_receive: bool,
}

/// 加载配置
pub fn load_config() -> Config {
    unimplemented!()
}

/// 保存配置
pub fn save_config(config: &Config) {
    unimplemented!()
}
