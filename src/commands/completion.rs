use crate::error::Result;
use clap_complete::{generate, Shell};
use std::io;

// Cli struct'ı main.rs'de olduğu için buradan erişemiyoruz
// Bu yüzden completion'ı main.rs'de handle edeceğiz
pub fn generate_completion(shell: Shell, cmd: &mut clap::Command) {
    let bin_name = "hermes";
    generate(shell, cmd, bin_name, &mut io::stdout());
}

pub fn execute(shell: Shell) -> Result<()> {
    // Bu fonksiyon main.rs'den Cli'yi alacak
    println!("# Run this command to generate completion:");
    println!("# hermes completion {}", shell_name(shell));
    Ok(())
}

fn shell_name(shell: Shell) -> &'static str {
    match shell {
        Shell::Bash => "bash",
        Shell::Zsh => "zsh",
        Shell::Fish => "fish",
        Shell::PowerShell => "powershell",
        Shell::Elvish => "elvish",
        _ => "unknown",
    }
}

pub fn print_install_instructions(shell: Shell) {
    println!("\n# Installation Instructions:");

    match shell {
        Shell::Bash => {
            println!("# Add to ~/.bashrc:");
            println!("eval \"$(hermes completion bash)\"");
            println!("\n# Or install globally:");
            println!("hermes completion bash | sudo tee /etc/bash_completion.d/hermes");
        }
        Shell::Zsh => {
            println!("# Add to ~/.zshrc:");
            println!("eval \"$(hermes completion zsh)\"");
            println!("\n# Or install to fpath:");
            println!("hermes completion zsh > ~/.zsh/completion/_hermes");
        }
        Shell::Fish => {
            println!("# Install to fish completions:");
            println!("hermes completion fish > ~/.config/fish/completions/hermes.fish");
        }
        Shell::PowerShell => {
            println!("# Add to PowerShell profile:");
            println!("hermes completion powershell | Out-String | Invoke-Expression");
            println!("\n# Or save to profile:");
            println!("hermes completion powershell >> $PROFILE");
        }
        Shell::Elvish => {
            println!("# Add to ~/.elvish/rc.elv:");
            println!("eval (hermes completion elvish | slurp)");
        }
        _ => {}
    }
}
