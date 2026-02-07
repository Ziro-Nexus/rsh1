
mod cli_config;
mod raw_mode;
mod utils;
mod rsh_builtin;

use std::io;
use cli_config::config::RshConfig;
use raw_mode::rsh_raw::rsh_shell;
use xiro;

use xiro_ai_lab::ai_manager;
use xiro_ai_lab::brain;

use tokio;



#[tokio::main]
async fn main() -> io::Result<()> {

    xiro::utils::telemetry::init_xiro_telemetry();
    let mut brain = brain::neural_system::XiroAIBrain::new(
        ai_manager::consumer::Communicator::new(
            "https://api.fireworks.ai/inference/v1/chat/completions".to_string(),
            "fw_G5XJY6swq6VxUp4TukzQww".to_string(),
            "accounts/fireworks/models/gpt-oss-20b".to_string()
        )
    );

    let mut mem_table = xiro::memory_table::vartable::VariableTableInMemory::new();

    let config = RshConfig {
        prompt: String::from("=> "),
        bin_path: String::from("/usr/bin").into()
    };
    rsh_shell(config, &mut mem_table, &mut brain).await?;
    Ok(())
}
