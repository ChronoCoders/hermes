use clap::{Parser, Subcommand};
use hermes::commands;
use hermes::error::Result;
use hermes::ui;

#[derive(Parser)]
#[command(name = "hermes")]
#[command(about = "Military-grade secure file transfer system", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Initialize Hermes configuration")]
    Init,

    #[command(about = "Display current configuration")]
    Config,

    #[command(about = "List all encrypted files in vault")]
    List,

    #[command(about = "Encrypt and send a text message")]
    SendMsg {
        #[arg(help = "Message to encrypt")]
        message: String,

        #[arg(short, long, help = "Encryption password")]
        password: String,

        #[arg(short = 'r', long, help = "Custom remote path")]
        remote_path: Option<String>,

        #[arg(
            short = 't',
            long,
            help = "Time-to-live in hours (self-destruct timer)"
        )]
        ttl: Option<u64>,
    },

    #[command(about = "Receive and decrypt a text message")]
    RecvMsg {
        #[arg(help = "Remote encrypted file name")]
        remote_file: String,

        #[arg(short, long, help = "Decryption password")]
        password: String,
    },

    #[command(about = "Encrypt and send a file")]
    SendFile {
        #[arg(help = "Path to file to encrypt")]
        file_path: String,

        #[arg(short, long, help = "Encryption password")]
        password: String,

        #[arg(short = 'r', long, help = "Custom remote path")]
        remote_path: Option<String>,

        #[arg(
            short = 't',
            long,
            help = "Time-to-live in hours (self-destruct timer)"
        )]
        ttl: Option<u64>,
    },

    #[command(about = "Receive and decrypt a file")]
    RecvFile {
        #[arg(help = "Remote encrypted file name")]
        remote_file: String,

        #[arg(short, long, help = "Decryption password")]
        password: String,

        #[arg(short, long, help = "Output file path")]
        output: Option<String>,
    },
}

fn main() -> Result<()> {
    ui::print_banner();

    let cli = Cli::parse();

    match cli.command {
        Commands::Init => {
            commands::init::execute()?;
        }
        Commands::Config => {
            commands::config::execute()?;
        }
        Commands::List => {
            commands::list::execute()?;
        }
        Commands::SendMsg {
            message,
            password,
            remote_path,
            ttl,
        } => {
            commands::send_msg::execute(&message, &password, remote_path.as_deref(), ttl)?;
        }
        Commands::RecvMsg {
            remote_file,
            password,
        } => {
            commands::recv_msg::execute(&remote_file, &password)?;
        }
        Commands::SendFile {
            file_path,
            password,
            remote_path,
            ttl,
        } => {
            commands::send_file::execute(&file_path, &password, remote_path.as_deref(), ttl)?;
        }
        Commands::RecvFile {
            remote_file,
            password,
            output,
        } => {
            commands::recv_file::execute(&remote_file, &password, output.as_deref())?;
        }
    }

    Ok(())
}
