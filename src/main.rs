use clap::{Parser, Subcommand};
use hermes::{commands, config, ui, Result};

#[derive(Parser)]
#[command(name = "hermes")]
#[command(version = "1.0.0")]
#[command(about = "Secure file transfer with military-grade encryption", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Encrypt and transmit text message")]
    SendMsg {
        #[arg(help = "Message to encrypt and send")]
        message: String,

        #[arg(short, long, help = "Encryption password")]
        password: String,
    },

    #[command(about = "Download and decrypt text message")]
    RecvMsg {
        #[arg(help = "Remote file name or path")]
        remote_file: String,

        #[arg(short, long, help = "Decryption password")]
        password: String,
    },

    #[command(about = "Encrypt and upload file")]
    SendFile {
        #[arg(help = "File path to encrypt and send")]
        file: String,

        #[arg(short, long, help = "Encryption password")]
        password: String,
    },

    #[command(about = "Download and decrypt file")]
    RecvFile {
        #[arg(help = "Remote file name or path")]
        remote_file: String,

        #[arg(short, long, help = "Decryption password")]
        password: String,

        #[arg(short, long, help = "Output file path")]
        output: Option<String>,
    },

    #[command(about = "Initialize Hermes configuration")]
    Init,

    #[command(about = "Show current configuration")]
    Config,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    ui::print_banner();

    match cli.command {
        Commands::SendMsg { message, password } => {
            commands::send_msg::execute(&message, &password)?;
        }

        Commands::RecvMsg {
            remote_file,
            password,
        } => {
            commands::recv_msg::execute(&remote_file, &password)?;
        }

        Commands::SendFile { file, password } => {
            commands::send_file::execute(&file, &password)?;
        }

        Commands::RecvFile {
            remote_file,
            password,
            output,
        } => {
            commands::recv_file::execute(&remote_file, &password, output.as_deref())?;
        }

        Commands::Init => {
            let config = config::Settings::default_config();
            config.save()?;
            ui::print_success("Configuration initialized");
            ui::print_info(
                "Location",
                &format!(
                    "{:?}",
                    dirs::config_dir()
                        .unwrap()
                        .join("hermes")
                        .join("config.toml")
                ),
            );
            println!();
        }

        Commands::Config => {
            let config = config::Settings::load()?;
            println!("{:#?}", config);
        }
    }

    Ok(())
}
