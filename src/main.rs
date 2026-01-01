
mod cli_config;
mod raw_mode;

use std::io;

use cli_config::config::RshConfig;
use raw_mode::rsh_raw::rsh_raw_mode;

fn main() -> io::Result<()> {
    let config = RshConfig {
        prompt: String::from("=> "),
        bin_path: String::from("/usr/bin").into()
    };
    rsh_raw_mode(config)?;
    Ok(())
}