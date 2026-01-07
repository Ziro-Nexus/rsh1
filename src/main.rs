
mod cli_config;
mod raw_mode;
mod utils;

use std::io;
use cli_config::config::RshConfig;
use raw_mode::rsh_raw::rsh_shell;
use xiro;

fn main() -> io::Result<()> {

    xiro::utils::telemetry::init_xiro_telemetry();
    let mut mem_table = xiro::memory_table::vartable::VariableTableInMemory::new();

    let config = RshConfig {
        prompt: String::from("=> "),
        bin_path: String::from("/usr/bin").into()
    };
    rsh_shell(config, &mut mem_table)?;
    Ok(())
}
