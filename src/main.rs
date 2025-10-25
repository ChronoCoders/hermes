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

    #[command(about = "Generate RSA keypair")]
    Keygen {
        #[arg(help = "Key name/identifier")]
        name: String,

        #[arg(short, long, help = "Output directory")]
        output: Option<String>,
    },

    #[command(about = "Import recipient's public key")]
    ImportPubkey {
        #[arg(help = "Recipient name/identifier")]
        name: String,

        #[arg(help = "Path to public key file")]
        pubkey: String,
    },

    #[command(about = "Export your public key")]
    ExportPubkey {
        #[arg(help = "Key name to export")]
        name: String,

        #[arg(short, long, help = "Output file path")]
        output: Option<String>,
    },

    #[command(about = "List all RSA keys")]
    ListKeys,

    #[command(about = "Encrypt and send a text message")]
    SendMsg {
        #[arg(help = "Message to encrypt")]
        message: String,

        #[arg(short, long, help = "Encryption password (if not using recipients)")]
        password: Option<String>,

        #[arg(short = 'r', long, help = "Custom remote path")]
        remote_path: Option<String>,

        #[arg(
            short = 't',
            long,
            help = "Time-to-live in hours (self-destruct timer)"
        )]
        ttl: Option<u64>,

        #[arg(long, value_delimiter = ',', help = "Recipients (comma-separated)")]
        recipients: Option<Vec<String>>,
    },

    #[command(about = "Receive and decrypt a text message")]
    RecvMsg {
        #[arg(help = "Remote encrypted file name")]
        remote_file: String,

        #[arg(short, long, help = "Decryption password (if not using recipient key)")]
        password: Option<String>,

        #[arg(long, help = "Recipient name (for multi-recipient messages)")]
        recipient: Option<String>,
    },

    #[command(about = "Encrypt and send a file")]
    SendFile {
        #[arg(help = "Path to file to encrypt")]
        file_path: String,

        #[arg(short, long, help = "Encryption password (if not using recipients)")]
        password: Option<String>,

        #[arg(short = 'r', long, help = "Custom remote path")]
        remote_path: Option<String>,

        #[arg(
            short = 't',
            long,
            help = "Time-to-live in hours (self-destruct timer)"
        )]
        ttl: Option<u64>,

        #[arg(long, value_delimiter = ',', help = "Recipients (comma-separated)")]
        recipients: Option<Vec<String>>,
    },

    #[command(about = "Receive and decrypt a file")]
    RecvFile {
        #[arg(help = "Remote encrypted file name")]
        remote_file: String,

        #[arg(short, long, help = "Decryption password (if not using recipient key)")]
        password: Option<String>,

        #[arg(short, long, help = "Output file path")]
        output: Option<String>,

        #[arg(long, help = "Recipient name (for multi-recipient files)")]
        recipient: Option<String>,
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
        Commands::Keygen { name, output } => {
            commands::keygen::execute(&name, output.as_deref())?;
        }
        Commands::ImportPubkey { name, pubkey } => {
            commands::import_pubkey::execute(&name, &pubkey)?;
        }
        Commands::ExportPubkey { name, output } => {
            commands::export_pubkey::execute(&name, output.as_deref())?;
        }
        Commands::ListKeys => {
            commands::list_keys::execute()?;
        }
        Commands::SendMsg {
            message,
            password,
            remote_path,
            ttl,
            recipients,
        } => {
            commands::send_msg::execute(
                &message,
                password.as_deref(),
                remote_path.as_deref(),
                ttl,
                recipients,
            )?;
        }
        Commands::RecvMsg {
            remote_file,
            password,
            recipient,
        } => {
            commands::recv_msg::execute(&remote_file, password.as_deref(), recipient.as_deref())?;
        }
        Commands::SendFile {
            file_path,
            password,
            remote_path,
            ttl,
            recipients,
        } => {
            commands::send_file::execute(
                &file_path,
                password.as_deref(),
                remote_path.as_deref(),
                ttl,
                recipients,
            )?;
        }
        Commands::RecvFile {
            remote_file,
            password,
            output,
            recipient,
        } => {
            commands::recv_file::execute(
                &remote_file,
                password.as_deref(),
                output.as_deref(),
                recipient.as_deref(),
            )?;
        }
    }

    Ok(())
}
