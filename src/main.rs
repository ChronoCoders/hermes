use clap::{Parser, Subcommand, ValueEnum};
use hermes::commands;
use hermes::error::Result;
use hermes::ui;

#[derive(Parser)]
#[command(name = "hermes")]
#[command(about = "Military-grade secure file transfer system", long_about = None)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Clone, ValueEnum)]
pub enum Shell {
    Bash,
    Zsh,
    Fish,
    PowerShell,
    Elvish,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Initialize Hermes configuration")]
    Init,

    #[command(about = "Display current configuration")]
    Config {
        #[arg(long, help = "Validate configuration")]
        validate: bool,

        #[arg(long, help = "Test SFTP connection")]
        test: bool,
    },

    #[command(about = "List all encrypted files in vault")]
    List,

    #[command(about = "Generate shell completion script")]
    Completion {
        #[arg(value_enum, help = "Shell type")]
        shell: clap_complete::Shell,
    },

    #[command(about = "Validate configuration")]
    Validate {
        #[arg(long, help = "Test SFTP connection")]
        test_connection: bool,
    },

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

    // ============================================
    // NEW: INTERACTIVE MODE (v1.1.0)
    // ============================================
    #[command(about = "Launch interactive mode (TUI)")]
    Interactive,

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

    // ============================================
    // NEW: BATCH OPERATIONS (v1.1.0)
    // ============================================
    #[command(about = "Encrypt and send multiple files (batch operation)")]
    SendBatch {
        #[arg(help = "Paths to files to encrypt", required = true)]
        file_paths: Vec<String>,

        #[arg(short, long, help = "Encryption password (if not using recipients)")]
        password: Option<String>,

        #[arg(
            short = 't',
            long,
            help = "Time-to-live in hours (self-destruct timer)"
        )]
        ttl: Option<u64>,

        #[arg(long, value_delimiter = ',', help = "Recipients (comma-separated)")]
        recipients: Option<Vec<String>>,
    },

    #[command(about = "Encrypt and send entire directory")]
    SendDir {
        #[arg(help = "Path to directory")]
        dir_path: String,

        #[arg(short, long, help = "Encryption password (if not using recipients)")]
        password: Option<String>,

        #[arg(
            short = 't',
            long,
            help = "Time-to-live in hours (self-destruct timer)"
        )]
        ttl: Option<u64>,

        #[arg(long, value_delimiter = ',', help = "Recipients (comma-separated)")]
        recipients: Option<Vec<String>>,

        #[arg(long, help = "Recursive directory traversal")]
        recursive: bool,
    },

    #[command(about = "Receive and decrypt multiple files (batch operation)")]
    RecvBatch {
        #[arg(help = "Remote encrypted file names", required = true)]
        remote_files: Vec<String>,

        #[arg(short, long, help = "Decryption password (if not using recipient key)")]
        password: Option<String>,

        #[arg(short, long, help = "Output directory")]
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
        Commands::Config { validate, test } => {
            if validate || test {
                commands::validate::execute(test)?;
            } else {
                commands::config::execute()?;
            }
        }
        Commands::List => {
            commands::list::execute()?;
        }
        Commands::Completion { shell } => {
            use clap::CommandFactory;
            use clap_complete::generate;

            let mut cmd = Cli::command();
            generate(shell, &mut cmd, "hermes", &mut std::io::stdout());
        }
        Commands::Validate { test_connection } => {
            commands::validate::execute(test_connection)?;
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

        // ============================================
        // NEW: INTERACTIVE MODE (v1.1.0)
        // ============================================
        Commands::Interactive => {
            commands::interactive::execute()?;
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

        // ============================================
        // NEW: BATCH OPERATIONS (v1.1.0)
        // ============================================
        Commands::SendBatch {
            file_paths,
            password,
            ttl,
            recipients,
        } => {
            commands::send_batch::execute(file_paths, password.as_deref(), ttl, recipients)?;
        }

        Commands::SendDir {
            dir_path,
            password,
            ttl,
            recipients,
            recursive,
        } => {
            commands::send_dir::execute(
                &dir_path,
                password.as_deref(),
                ttl,
                recipients,
                recursive,
            )?;
        }

        Commands::RecvBatch {
            remote_files,
            password,
            output,
            recipient,
        } => {
            commands::recv_batch::execute(
                remote_files,
                password.as_deref(),
                output.as_deref(),
                recipient.as_deref(),
            )?;
        }
    }

    Ok(())
}
