use crossterm::{
    cursor::MoveToColumn,
    event::{self, Event, KeyCode},
    execute,
    terminal::Clear,
    terminal::ClearType,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use nix::sys::wait::waitpid;
use nix::{libc::execve, unistd::ForkResult, unistd::fork};
use std::ffi::CString;
use std::io::{self, Write};
use std::ptr;

use crate::{cli_config::config::RshConfig, utils::pre_processing_utils::expand_xiro_variables};

pub fn execute_program(stdout: &mut io::Stdout, config: &RshConfig, input: String) {
    if input.trim().is_empty() {
        return;
    }

    let parts: Vec<&str> = input.split_whitespace().collect();
    let mut bin_path = config.bin_path.clone();
    bin_path.push(parts[0]);
    let program = CString::new(bin_path.to_str().unwrap()).unwrap();

    let arg_strings: Vec<CString> = parts
        .iter()
        .map(|s| CString::new(*s).expect("Error al convertir a CString"))
        .collect();

    let mut argv: Vec<*const i8> = arg_strings.iter().map(|cs| cs.as_ptr()).collect();

    // (requisito de POSIX)
    argv.push(ptr::null());

    execute!(stdout, MoveToColumn(0), Clear(ClearType::CurrentLine)).ok();
    disable_raw_mode().ok();

    unsafe {
        match fork() {
            Ok(ForkResult::Child) => {
                let envp: [*const i8; 1] = [ptr::null()];
                let result = execve(program.as_ptr(), argv.as_ptr(), envp.as_ptr());

                println!("Returned code => {}", result);

                // Si execve falla, debemos salir del proceso hijo
                std::process::exit(1);
            }
            Ok(ForkResult::Parent { child }) => {
                let _ = waitpid(child, None);
            }
            Err(e) => eprintln!("fork failed: {}", e),
        }
    }
    enable_raw_mode().ok();
}

pub fn rsh_shell(
    config: RshConfig,
    mem_table: &mut xiro::memory_table::vartable::VariableTableInMemory,
) -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();

    let prompt = config.prompt.as_str();
    let mut input = String::new();
    let mut history: Vec<String> = Vec::new();
    let mut history_index: Option<usize> = None;
    print!("{}", prompt);
    stdout.flush()?;

    loop {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char(c) => {
                    input.push(c);
                }
                KeyCode::Backspace => {
                    input.pop();
                }
                // Modifica la lógica del KeyCode::Enter dentro de rsh_shell
                KeyCode::Enter => {
                    println!();
                    if input.is_empty() {
                        execute!(stdout, MoveToColumn(0), Clear(ClearType::CurrentLine))?;
                        print!("{}", prompt);
                        stdout.flush()?;
                        continue;
                    }

                    history.push(input.clone());
                    history_index = None;

                    execute!(stdout, MoveToColumn(0), Clear(ClearType::CurrentLine))?;
                    disable_raw_mode()?;
                    // 1. Intentar analizar como Xiro
                    let syntax_analyzer =
                        xiro::report::generator::generate_syntax_report(input.as_str());

                    // Si el generador detectó que es una operación válida de Xiro (Set o Def)
                    if syntax_analyzer.is_variable_declaration || syntax_analyzer.is_set_variable {
                        xiro::report::syntax_report_handler::ReportHandler::handle_report(
                            &syntax_analyzer,
                            mem_table,
                        );
                        //xiro::report::syntax_report_handler::ReportHandler::print_status_report(
                        //    &syntax_analyzer,
                        //);

                    } else {
                        // 2. Si no es Xiro puro, es un comando de Sistema o una mezcla
                        // Aquí ocurre la magia: PRE-PROCESAMIENTO DE VARIABLES
                        let expanded_input = expand_xiro_variables(input.clone(), mem_table);

                        execute_program(&mut stdout, &config, expanded_input);
                    }

                    input.clear();
                    enable_raw_mode()?;
                    execute!(stdout, MoveToColumn(0), Clear(ClearType::CurrentLine))?;
                    // Re-activar el estado visual
                    print!("{}", prompt);
                    stdout.flush()?;
                }
                KeyCode::Up => {
                    if history.is_empty() {
                        continue;
                    }

                    history_index = match history_index {
                        None => Some(history.len() - 1),
                        Some(0) => Some(0),
                        Some(i) => Some(i - 1),
                    };

                    input = history[history_index.unwrap()].clone();
                }
                KeyCode::Down => {
                    if let Some(i) = history_index {
                        if i + 1 < history.len() {
                            history_index = Some(i + 1);
                            input = history[i + 1].clone();
                        } else {
                            history_index = None;
                            input.clear();
                        }
                    }
                }
                KeyCode::Esc => break,
                _ => {}
            }
            execute!(stdout, MoveToColumn(0), Clear(ClearType::CurrentLine))?;
            print!("{}{}", prompt, input);
            stdout.flush()?;
        }
    }

    disable_raw_mode()?;
    Ok(())
}
