#[macro_export]
macro_rules! get_config_file_path {
    ($name:expr) => {
        format!(
            "{}/{}.setup.json",
            std::env::var("SETUP_ROOT_PATH").unwrap_or("./".to_string()),
            $name
        );
    };
}
