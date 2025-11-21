pub mod internal;
use internal::domain::event::Event;
use internal::services::tui::{App, key_inputs::handle_key_inputs};
use std::env;
use std::io::{BufRead, BufReader};
use std::process::{Child, Command, Stdio};
use std::{sync::mpsc, thread::spawn};

use crate::internal::domain::record::Record;
use crate::internal::services::tui::TuiError;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error(transparent)]
    Tui(#[from] TuiError),
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error(transparent)]
    Send(#[from] std::sync::mpsc::SendError<Event>),
}

// TODO:
// - shutdown properly (not kill)
fn main() -> Result<(), AppError> {
    // Set panic hook to be able to panic from another thread and still exit programm

    let orig_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        orig_hook(panic_info);
        std::process::exit(1);
    }));

    // Create channels

    let (rr_tx, rr_rx) = mpsc::channel::<Event>();

    let mut child_process: Option<Child> = None;

    if let Ok(mock) = env::var("MOCK")
        && mock
            .to_lowercase()
            .parse()
            .expect("should contain true/false, case is ignored")
    {
        let rr_tx_stdin = rr_tx.clone();
        spawn(move || {
            loop {
                || -> Result<(), AppError> {
                    for i in 1..37 {
                        let r = Record::new(format!(r#"{:?} - key=01556cbc-b520-48e5-b5c8-fa25b4211cc3, value="ID":"01556cbc-b520-48e5-b5c8-fa25b4211cc3","PrivateKey":"0x0x93ed6fbc2f7d1922fd8a3932a58ae713465d4dfb0db83fe74b21d523d015cb01","TelegramID":"279058397""#, i).to_string());
                        rr_tx_stdin.send(Event::StdIn(r))?;
                    }
                    // thread::sleep(std::time::Duration::from_secs(10000));
                    for i in 36..360 {
                        let r = Record::new(format!(r#"{:?} - key=01556cbc-b520-48e5-b5c8-fa25b4211cc3, value="ID":"01556cbc-b520-48e5-b5c8-fa25b4211cc3","PrivateKey":"0x0x93ed6fbc2f7d1922fd8a3932a58ae713465d4dfb0db83fe74b21d523d015cb01","TelegramID":"279058397""#, i).to_string());
                        rr_tx_stdin.send(Event::StdIn(r))?;
                        std::thread::sleep(std::time::Duration::from_secs(1));
                    }

                    Ok(())
                }()
                .expect("reading log from stdin");
            }
        });
    } else {
        let args: Vec<_> = std::env::args().collect(); // get all arguments passed to app

        match args.len() {
            1 => {
                // Reading log entries from stdin

                let rr_tx_stdin = rr_tx.clone();
                spawn(move || {
                    loop {
                        || -> Result<(), AppError> {
                            if atty::is(atty::Stream::Stdin) {
                                return Ok(());
                            }
                            let mut line = String::new();
                            std::io::stdin().read_line(&mut line)?;

                            rr_tx_stdin.send(Event::StdIn(line.into()))?;

                            Ok(())
                        }()
                        .expect("reading log from stdin");
                    }
                });
            }
            _ => {
                // Run child process

                || -> Result<(), AppError> {
                    let system_command = args[1].clone();
                    let arguments = args[2..].iter();
                    let current_dir = env::current_dir()?;
                    let mut child = Command::new(system_command)
                        .args(arguments)
                        .current_dir(current_dir)
                        .stdout(Stdio::piped())
                        .stderr(Stdio::piped())
                        .spawn()
                        .expect("should be able to execute `echo`");

                    let stdout = child.stdout.take();
                    if let Some(stdout) = stdout {
                        let rr_tx_stdout = rr_tx.clone();
                        spawn(move || {
                            let stdout_reader = BufReader::new(stdout);
                            let stdout_lines = stdout_reader.lines();

                            for line in stdout_lines {
                                rr_tx_stdout
                                    .send(Event::StdIn(line.unwrap().into()))
                                    .expect("couldn't send stdout event");
                            }
                        });
                    }

                    let stderr = child.stderr.take();
                    if let Some(stderr) = stderr {
                        let rr_tx_stderr = rr_tx.clone();
                        spawn(move || {
                            let stderr_reader = BufReader::new(stderr);
                            let stderr_lines = stderr_reader.lines();

                            for line in stderr_lines {
                                rr_tx_stderr
                                    .send(Event::StdIn(line.unwrap().into()))
                                    .expect("couldn't send stderr event");
                            }
                        });
                    }

                    child_process = Some(child);

                    Ok(())
                }()
                .expect("running subprocess");
            }
        }
    }

    // Init terminal

    let mut terminal = ratatui::init();

    // Reading key inputs

    let rr_tx_keys = rr_tx.clone();
    spawn(move || {
        handle_key_inputs(rr_tx_keys);
    });

    // Run

    let mut tui = App::new(rr_rx);
    let result = tui.run(&mut terminal);

    // Shutdown

    ratatui::restore();
    if let Some(mut cp) = child_process {
        cp.kill()?;
        cp.wait()?;
    }

    Ok(result?)
}
