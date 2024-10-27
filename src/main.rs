use clap::Parser;
use csmacros::cmd;
use log::{error, info, Level, LevelFilter};
use simplelog::{Color, ColorChoice, ConfigBuilder, TermLogger, TerminalMode};
use std::process::{exit, Command};

/// `cshell` is a small, simple Rust-based CLI that allows you to change your Linux shell effortlessly.
#[derive(Parser, Debug)]
#[command(
    name = "cshell",
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = "A simple CLI tool for changing your Linux shell",
    long_about = "cshell is designed to make switching between shells on Linux smooth and efficient, with options to guide you through the process."
)]
struct Cshell {
    /// Activates tutorial mode, guiding the user through changing shells.
    #[arg(short, long, help = "Run in tutorial mode")]
    tutor: bool,

    /// The name of the shell to switch to (e.g., bash, zsh, nu).
    #[arg(help = "The shell to change to: bash, zsh, or nu")]
    shell: Option<String>,
}

impl Cshell {
    /// Runs the shell-changing functionality based on the shell name provided.
    pub fn change_shell(&self) {
        // return the output
        let shell_name = match &self.shell {
            Some(name) => name,
            None => {
                error!("Error: No shell specified.");
                exit(1);
            }
        };

        let shell_path = match shell_name.as_str() {
            "bash" => "/bin/bash",
            "zsh" => "/usr/bin/zsh",
            "nu" => "/usr/bin/nu",
            _ => {
                error!("Error: Unsupported shell '{}'", shell_name);
                exit(1);
            }
        };

        info!("Switching to shell: {}", shell_name);

        // Attempt to change the shell using `chsh`
        let status = Command::new("chsh").arg("-s").arg(shell_path).status();

        match status {
            Ok(status) if status.success() => {
                info!("Shell successfully changed to {}.", shell_name);
                info!("Please log out and log back in to apply the changes.");
            }
            Ok(_) => {
                error!("Failed to change shell: Permission denied or invalid shell.");
                exit(1);
            }
            Err(e) => {
                error!("Error executing command: {}", e);
                exit(1);
            }
        }
    }

    /// Runs a tutorial if `tutor` mode is active.
    pub fn run_tutorial(&self) {
        // ANSI color codes
        let green_bold = "\x1b[1;32m";
        let cyan = "\x1b[0;36m";
        let yellow = "\x1b[0;33m";
        let reset = "\x1b[0m";

        println!("{}Entering tutorial mode...{}", green_bold, reset);
        println!(
            "{}In Linux, you can change your shell using `chsh`.{}",
            cyan, reset
        );
        println!(
            "{}This program helps you do that with an easy-to-use CLI.{}",
            cyan, reset
        );
        println!(
            "{}Example usage:\n\n\tcshell --tutor\n\tcshell zsh\n\tcshell bash{}",
            yellow, reset
        );
        println!("{}Supported shells: bash, zsh, nu.{}", cyan, reset);
    }
}

fn main() {
    let args = Cshell::parse();

    let config = ConfigBuilder::new()
        .set_level_color(Level::Error, Some(Color::Red))
        .set_level_color(Level::Info, Some(Color::Green))
        .set_target_level(LevelFilter::Trace)
        .add_filter_allow_str("cshell")
        .set_location_level(LevelFilter::Off)
        .build();

    TermLogger::init(
        LevelFilter::Trace,
        config,
        TerminalMode::Stdout,
        ColorChoice::Auto,
    )
    .unwrap();

    if args.tutor {
        args.run_tutorial();
    } else {
        args.change_shell();
    }

    // cmd!("Hello", "cool");
}
