use crate::commands;
use crate::error::Result;
use crate::ui;
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};

pub fn execute() -> Result<()> {
    ui::print_banner();

    println!("{}", "═".repeat(60).bright_cyan());
    println!("{}", "🎮 INTERACTIVE MODE".bright_white().bold());
    println!("{}", "═".repeat(60).bright_cyan());
    println!();

    loop {
        let options = vec![
            "📤 Send Message",
            "📥 Receive Message",
            "📄 Send File",
            "📂 Receive File",
            "📦 Batch Operations",
            "🔑 Key Management",
            "⚙️  Configuration",
            "📋 List Vault",
            "❌ Exit",
        ];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select an option")
            .items(&options)
            .default(0)
            .interact()?;

        println!();

        match selection {
            0 => send_message_wizard()?,
            1 => receive_message_wizard()?,
            2 => send_file_wizard()?,
            3 => receive_file_wizard()?,
            4 => batch_operations_wizard()?,
            5 => key_management_wizard()?,
            6 => configuration_wizard()?,
            7 => commands::list::execute()?,
            8 => {
                println!("{}", "👋 Goodbye!".bright_green());
                break;
            }
            _ => unreachable!(),
        }

        println!();
    }

    Ok(())
}

fn send_message_wizard() -> Result<()> {
    println!("{}", "📤 SEND MESSAGE".bright_yellow().bold());
    println!();

    let message: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter message to encrypt")
        .interact_text()?;

    let encryption_type = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select encryption type")
        .items(&["Password", "Recipients (RSA)"])
        .default(0)
        .interact()?;

    let (password, recipients) = if encryption_type == 0 {
        let pwd: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter password")
            .interact_text()?;
        (Some(pwd), None)
    } else {
        let recips: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter recipients (comma-separated)")
            .interact_text()?;
        let recip_vec: Vec<String> = recips.split(',').map(|s| s.trim().to_string()).collect();
        (None, Some(recip_vec))
    };

    let use_ttl = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Set expiration time? (self-destruct)")
        .default(false)
        .interact()?;

    let ttl = if use_ttl {
        let hours: u64 = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Expiration time (hours)")
            .default(24)
            .interact_text()?;
        Some(hours)
    } else {
        None
    };

    println!();
    commands::send_msg::execute(&message, password.as_deref(), None, ttl, recipients)?;

    Ok(())
}

fn receive_message_wizard() -> Result<()> {
    println!("{}", "📥 RECEIVE MESSAGE".bright_yellow().bold());
    println!();

    let remote_file: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter encrypted message filename")
        .interact_text()?;

    let decryption_type = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select decryption type")
        .items(&["Password", "Recipient Key"])
        .default(0)
        .interact()?;

    let (password, recipient) = if decryption_type == 0 {
        let pwd: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter password")
            .interact_text()?;
        (Some(pwd), None)
    } else {
        let recip: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter recipient name")
            .interact_text()?;
        (None, Some(recip))
    };

    println!();
    commands::recv_msg::execute(&remote_file, password.as_deref(), recipient.as_deref())?;

    Ok(())
}

fn send_file_wizard() -> Result<()> {
    println!("{}", "📄 SEND FILE".bright_yellow().bold());
    println!();

    let file_path: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter file path")
        .interact_text()?;

    let encryption_type = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select encryption type")
        .items(&["Password", "Recipients (RSA)"])
        .default(0)
        .interact()?;

    let (password, recipients) = if encryption_type == 0 {
        let pwd: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter password")
            .interact_text()?;
        (Some(pwd), None)
    } else {
        let recips: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter recipients (comma-separated)")
            .interact_text()?;
        let recip_vec: Vec<String> = recips.split(',').map(|s| s.trim().to_string()).collect();
        (None, Some(recip_vec))
    };

    let use_ttl = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Set expiration time? (self-destruct)")
        .default(false)
        .interact()?;

    let ttl = if use_ttl {
        let hours: u64 = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Expiration time (hours)")
            .default(24)
            .interact_text()?;
        Some(hours)
    } else {
        None
    };

    println!();
    commands::send_file::execute(&file_path, password.as_deref(), None, ttl, recipients, None)?;

    Ok(())
}

fn receive_file_wizard() -> Result<()> {
    println!("{}", "📂 RECEIVE FILE".bright_yellow().bold());
    println!();

    let remote_file: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter encrypted file name")
        .interact_text()?;

    let decryption_type = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select decryption type")
        .items(&["Password", "Recipient Key"])
        .default(0)
        .interact()?;

    let (password, recipient) = if decryption_type == 0 {
        let pwd: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter password")
            .interact_text()?;
        (Some(pwd), None)
    } else {
        let recip: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter recipient name")
            .interact_text()?;
        (None, Some(recip))
    };

    let use_output = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Specify output path?")
        .default(false)
        .interact()?;

    let output = if use_output {
        let path: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter output path")
            .interact_text()?;
        Some(path)
    } else {
        None
    };

    println!();
    commands::recv_file::execute(
        &remote_file,
        password.as_deref(),
        output.as_deref(),
        recipient.as_deref(),
    )?;

    Ok(())
}

fn batch_operations_wizard() -> Result<()> {
    println!("{}", "📦 BATCH OPERATIONS".bright_yellow().bold());
    println!();

    let batch_type = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select batch operation")
        .items(&[
            "Send Multiple Files",
            "Send Directory",
            "Receive Multiple Files",
        ])
        .default(0)
        .interact()?;

    match batch_type {
        0 => send_batch_wizard()?,
        1 => send_dir_wizard()?,
        2 => recv_batch_wizard()?,
        _ => unreachable!(),
    }

    Ok(())
}

fn send_batch_wizard() -> Result<()> {
    println!("{}", "📤 SEND BATCH".bright_cyan().bold());
    println!();

    let files_input: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter file paths (comma-separated)")
        .interact_text()?;

    let file_paths: Vec<String> = files_input
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();

    let encryption_type = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select encryption type")
        .items(&["Password", "Recipients (RSA)"])
        .default(0)
        .interact()?;

    let (password, recipients) = if encryption_type == 0 {
        let pwd: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter password")
            .interact_text()?;
        (Some(pwd), None)
    } else {
        let recips: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter recipients (comma-separated)")
            .interact_text()?;
        let recip_vec: Vec<String> = recips.split(',').map(|s| s.trim().to_string()).collect();
        (None, Some(recip_vec))
    };

    let use_ttl = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Set expiration time?")
        .default(false)
        .interact()?;

    let ttl = if use_ttl {
        let hours: u64 = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Expiration time (hours)")
            .default(24)
            .interact_text()?;
        Some(hours)
    } else {
        None
    };

    println!();
    commands::send_batch::execute(file_paths, password.as_deref(), ttl, recipients)?;

    Ok(())
}

fn send_dir_wizard() -> Result<()> {
    println!("{}", "📁 SEND DIRECTORY".bright_cyan().bold());
    println!();

    let dir_path: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter directory path")
        .interact_text()?;

    let recursive = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Recursive (include subdirectories)?")
        .default(true)
        .interact()?;

    let encryption_type = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select encryption type")
        .items(&["Password", "Recipients (RSA)"])
        .default(0)
        .interact()?;

    let (password, recipients) = if encryption_type == 0 {
        let pwd: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter password")
            .interact_text()?;
        (Some(pwd), None)
    } else {
        let recips: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter recipients (comma-separated)")
            .interact_text()?;
        let recip_vec: Vec<String> = recips.split(',').map(|s| s.trim().to_string()).collect();
        (None, Some(recip_vec))
    };

    let use_ttl = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Set expiration time?")
        .default(false)
        .interact()?;

    let ttl = if use_ttl {
        let hours: u64 = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Expiration time (hours)")
            .default(24)
            .interact_text()?;
        Some(hours)
    } else {
        None
    };

    println!();
    commands::send_dir::execute(&dir_path, password.as_deref(), ttl, recipients, recursive)?;

    Ok(())
}

fn recv_batch_wizard() -> Result<()> {
    println!("{}", "📥 RECEIVE BATCH".bright_cyan().bold());
    println!();

    let files_input: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter encrypted file names (comma-separated)")
        .interact_text()?;

    let remote_files: Vec<String> = files_input
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();

    let decryption_type = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select decryption type")
        .items(&["Password", "Recipient Key"])
        .default(0)
        .interact()?;

    let (password, recipient) = if decryption_type == 0 {
        let pwd: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter password")
            .interact_text()?;
        (Some(pwd), None)
    } else {
        let recip: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter recipient name")
            .interact_text()?;
        (None, Some(recip))
    };

    let use_output = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Specify output directory?")
        .default(false)
        .interact()?;

    let output = if use_output {
        let path: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter output directory")
            .interact_text()?;
        Some(path)
    } else {
        None
    };

    println!();
    commands::recv_batch::execute(
        remote_files,
        password.as_deref(),
        output.as_deref(),
        recipient.as_deref(),
    )?;

    Ok(())
}

fn key_management_wizard() -> Result<()> {
    println!("{}", "🔑 KEY MANAGEMENT".bright_yellow().bold());
    println!();

    let key_options = vec![
        "Generate New Keypair",
        "Import Public Key",
        "Export Public Key",
        "List All Keys",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select key operation")
        .items(&key_options)
        .default(0)
        .interact()?;

    println!();

    match selection {
        0 => {
            let name: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter key name")
                .interact_text()?;

            println!();
            commands::keygen::execute(&name, None)?;
        }
        1 => {
            let name: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter recipient name")
                .interact_text()?;

            let pubkey_path: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter public key file path")
                .interact_text()?;

            println!();
            commands::import_pubkey::execute(&name, &pubkey_path)?;
        }
        2 => {
            let name: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter key name to export")
                .interact_text()?;

            let use_output = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Specify output path?")
                .default(false)
                .interact()?;

            let output = if use_output {
                let path: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter output file path")
                    .interact_text()?;
                Some(path)
            } else {
                None
            };

            println!();
            commands::export_pubkey::execute(&name, output.as_deref())?;
        }
        3 => {
            commands::list_keys::execute()?;
        }
        _ => unreachable!(),
    }

    Ok(())
}

fn configuration_wizard() -> Result<()> {
    println!("{}", "⚙️  CONFIGURATION".bright_yellow().bold());
    println!();

    let config_options = vec![
        "View Configuration",
        "Validate Configuration",
        "Test SFTP Connection",
        "Reinitialize Configuration",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select configuration option")
        .items(&config_options)
        .default(0)
        .interact()?;

    println!();

    match selection {
        0 => {
            commands::config::execute()?;
        }
        1 => {
            commands::validate::execute(false)?;
        }
        2 => {
            commands::validate::execute(true)?;
        }
        3 => {
            let confirm = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Reinitialize configuration? (This will overwrite existing config)")
                .default(false)
                .interact()?;

            if confirm {
                commands::init::execute()?;
            }
        }
        _ => unreachable!(),
    }

    Ok(())
}
