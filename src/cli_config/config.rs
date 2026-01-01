use std::path::PathBuf;


pub struct RshConfig {
    pub prompt: String,
    pub bin_path: PathBuf
}


impl RshConfig {
    pub fn new(prompt: String, bin_path: String) -> Self {
        let bin_path = PathBuf::from(bin_path);
        Self {
            prompt,
            bin_path
        }
    }
}
