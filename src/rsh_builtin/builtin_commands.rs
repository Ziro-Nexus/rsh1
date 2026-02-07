use std::fs;
use xiro_ai_lab::brain;
use colored::*; // Necesitarás añadir `colored = "2"` a tu Cargo.toml

fn print_side_by_side(ascii_path: &str, message: &str) {
    let ascii_art = fs::read_to_string(ascii_path).unwrap_or_else(|_| " [AI] ".to_string());
    let terminal_width = 50; // Ajusta según el ancho de tu ASCII

    let ascii_lines: Vec<&str> = ascii_art.lines().collect();

    // Dividimos el mensaje de la IA en fragmentos para que no desborde la pantalla
    let wrapped_message = textwrap::fill(message, 60); // Necesitas la crate 'textwrap'
    let msg_lines: Vec<&str> = wrapped_message.lines().collect();

    let max_lines = std::cmp::max(ascii_lines.len(), msg_lines.len());

    println!("\n{}", "── TRANSMISIÓN ENTRANTE ──────────────────────────────────".dimmed());
    for i in 0..max_lines {
        let ascii_part = ascii_lines.get(i).unwrap_or(&"");
        let msg_part = msg_lines.get(i).unwrap_or(&"");
        // Imprime la línea del ASCII en cyan y el mensaje en blanco brillante
        println!(
            "{:width$}   {}",
            ascii_part.bright_cyan(),
            msg_part.bright_white().bold(),
            width = terminal_width
        );
    }
    println!("{}\n", "──────────────────────────────────────────────────────────".dimmed());
}

pub fn check_is_command(input: &str) -> bool {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return false;
    }
    let first_word = trimmed.split_whitespace().next().unwrap();
    matches!(first_word, "@create_neuron" | "@talk" | "@help" | "@list_neurons")
}

pub async fn map_and_run_cmd(input: &str, brain: &mut brain::neural_system::XiroAIBrain) {
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    let command = parts.get(0).cloned();

    match command {
        Some("@create_neuron") => {
            if parts.len() < 3 {
                println!("{}", " ✘ Error ".on_red().white().bold());
                println!("{}", "Use: @create_neuron <session_name> <context_path>".red());
                return;
            }

            let session_name = parts[1];
            let context_path = parts[2..].join(" ");

            println!(" {} Creating neuron: {}...", "⚙".cyan().bold(), session_name.bright_yellow());

            match fs::read_to_string(&context_path) {
                Ok(context) => {
                    brain.create_session(session_name.to_owned(), context.clone()).await;
                    println!(
                        " {} {}",
                        "✔".green().bold(),
                        "Neuron created succesfully".bright_green()
                    );
                    println!("   {} {}", "ID:".dimmed(), session_name.underline());
                }
                Err(e) => {
                    println!(" {} {}: {}", "⚠".red().bold(), "Failed to read context".red(), e);
                }
            }
        }

        Some("@list_neurons") => {
            println!("\n{}", " 🧠 ACTIVE BRAIN NETWORKS".on_bright_magenta().white().bold());
            let sessions = brain.list_sessions();

            if sessions.is_empty() {
                println!(" {}", "Neurons are empty".dimmed());
            } else {
                for session in sessions {
                    if let Some(s_data) = brain.sessions.get(&session) {
                        println!(
                            " {} {:<15} {} {}",
                            "•".bright_magenta(),
                            s_data.session_name.bright_white(),
                            "→".dimmed(),
                            s_data.session_id.dimmed()
                        );
                    }
                }
            }
            println!("");
        }

        Some("@talk") => {
            if parts.len() < 3 {
                println!(" {} {}", "⚠".yellow(), "Use: @talk <id> <msg>".yellow());
                return;
            }

            let session_id = parts[1];
            let msg = parts[2..].join(" ");

            use std::io::{ self, Write };
            io::stdout().flush().unwrap();

            match brain.talk(session_id.to_string(), msg).await {
                Some(response) => {
                    print!("\r"); // Limpia el mensaje de carga
                    // Aquí llamamos a la función mágica
                    print_side_by_side("/home/xiro/.local/rsh/context_art.txt", &response);
                }
                None => println!("\r {} Error communicating with the AI", "⚡".red()),
            }
        }

        Some("@help") => {
            show_pretty_help();
        }

        _ => {
            println!(" {} Cmd {} not exist", "❓".yellow(), input.red());
        }
    }
}

fn show_pretty_help() {
    println!("\n{}", " ✨ XIRO AI LAB - TERMINAL INTERFACE ✨ ".on_blue().white().bold());
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".blue());
    println!(
        "  {}  {}  {}",
        "@create_neuron".cyan(),
        "<name> <path>".dimmed(),
        "Create a new agent instance"
    );
    println!(
        "  {}  {}  {}",
        "@talk".cyan(),
        "<id> <msg>".dimmed(),
        "Interactive with an agent instance"
    );
    println!(
        "  {}  {}  {}",
        "@list_neurons".cyan(),
        "".dimmed(),
        "List all active agent instances"
    );
    println!("  {}  {}  {}", "@help".cyan(), "".dimmed(), "Show this help message");
    println!("{}\n", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".blue());
}
