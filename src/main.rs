use clap::{Parser, Subcommand};
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

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Initialize Hermes configuration")]
    Init,

    #[command(about = "Display current configuration")]
    Config,

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
        #[arg(help = "Name/identifier for this keypair")]
        name: String,

        #[arg(short, long, help = "Output directory for keys")]
        output: Option<String>,

        #[arg(long, help = "Generate hybrid keypair with post-quantum Kyber")]
        pqc: bool,
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
        #[arg(help = "Your keypair name")]
        name: String,

        #[arg(short, long, help = "Output file path")]
        output: Option<String>,
    },

    #[command(about = "Import recipient's Kyber (PQC) public key")]
    ImportKyberPubkey {
        #[arg(help = "Recipient name/identifier")]
        name: String,

        #[arg(help = "Path to Kyber public key file")]
        pubkey: String,
    },

    #[command(about = "Export your Kyber (PQC) public key")]
    ExportKyberPubkey {
        #[arg(help = "Your keypair name")]
        name: String,

        #[arg(short, long, help = "Output file path")]
        output: Option<String>,
    },

    #[command(about = "List all RSA keys")]
    ListKeys,

    #[command(about = "Split RSA private key into shares (Shamir's Secret Sharing)")]
    KeySplit {
        #[arg(help = "Path to private key file")]
        name: String,

        #[arg(short = 't', long, help = "Threshold (minimum shares needed to recover)")]
        threshold: u8,

        #[arg(short = 'n', long, help = "Total number of shares to create")]
        shares: u8,

        #[arg(short, long, help = "Output directory for shares")]
        output: Option<String>,
    },

    #[command(about = "Recover RSA private key from shares")]
    KeyRecover {
        #[arg(help = "Paths to share files", required = true)]
        share_paths: Vec<String>,

        #[arg(short = 'n', long, help = "Output name for recovered key")]
        name: String,
    },

    #[command(about = "Verify a share file's integrity")]
    ShareVerify {
        #[arg(help = "Path to share file")]
        share_path: String,
    },

    #[command(about = "Check-in to prevent file deletion (Dead Man's Switch)")]
    Checkin {
        #[arg(help = "Remote file path")]
        file_path: String,
    },

    #[command(about = "Show Dead Man's Switch status for all files")]
    DmsStatus,

    #[command(about = "Disable Dead Man's Switch for a file")]
    DmsDisable {
        #[arg(help = "Remote file path")]
        file_path: String,
    },

    #[command(about = "Launch interactive mode (TUI)")]
    Interactive,

    #[command(about = "Encrypt and send a text message")]
    SendMsg {
        #[arg(help = "Message text to encrypt")]
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
        #[arg(help = "Remote encrypted message name")]
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

        #[arg(long, help = "Dead Man's Switch timeout in hours")]
        dms: Option<u64>,

        #[arg(long, help = "Use post-quantum hybrid encryption (requires PQC keys)")]
        pqc: bool,
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

    #[command(about = "Encrypt and send large file in chunks (memory-efficient)")]
    SendFileChunked {
        #[arg(help = "Path to large file to encrypt")]
        file_path: String,

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

    #[command(about = "Receive and decrypt chunked file")]
    RecvFileChunked {
        #[arg(help = "Remote manifest file name")]
        remote_manifest: String,

        #[arg(short, long, help = "Decryption password (if not using recipient key)")]
        password: Option<String>,

        #[arg(short, long, help = "Output file path")]
        output: Option<String>,

        #[arg(long, help = "Recipient name (for multi-recipient files)")]
        recipient: Option<String>,
    },

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
        #[arg(help = "Path to directory to encrypt")]
        dir_path: String,

        #[arg(short, long, help = "Encryption password (if not using recipients)")]
        password: Option<String>,

        #[arg(short, long, help = "Include subdirectories")]
        recursive: bool,

        #[arg(
            short = 't',
            long,
            help = "Time-to-live in hours (self-destruct timer)"
        )]
        ttl: Option<u64>,

        #[arg(long, value_delimiter = ',', help = "Recipients (comma-separated)")]
        recipients: Option<Vec<String>>,
    },

    #[command(about = "Receive and decrypt multiple files (batch operation)")]
    RecvBatch {
        #[arg(help = "Pattern or list of remote files", required = true)]
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
        Commands::Config => {
            commands::config::execute()?;
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
        Commands::Keygen { name, output, pqc } => {
            commands::keygen::execute(&name, output.as_deref(), pqc)?;
        }
        Commands::ImportPubkey { name, pubkey } => {
            commands::import_pubkey::execute(&name, &pubkey)?;
        }
        Commands::ExportPubkey { name, output } => {
            commands::export_pubkey::execute(&name, output.as_deref())?;
        }
        Commands::ImportKyberPubkey { name, pubkey } => {
            commands::import_kyber_pubkey::execute(&name, &pubkey)?;
        }
        Commands::ExportKyberPubkey { name, output } => {
            commands::export_kyber_pubkey::execute(&name, output.as_deref())?;
        }
        Commands::ListKeys => {
            commands::list_keys::execute()?;
        }
        Commands::KeySplit {
            name,
            threshold,
            shares,
            output,
        } => {
            commands::key_split::execute(&name, threshold, shares, output.as_deref())?;
        }
        Commands::KeyRecover { share_paths, name } => {
            commands::key_recover::execute(share_paths, &name)?;
        }
        Commands::ShareVerify { share_path } => {
            commands::share_verify::execute(&share_path)?;
        }
        Commands::Checkin { file_path } => {
            commands::checkin::execute(&file_path)?;
        }
        Commands::DmsStatus => {
            commands::dms_status::execute()?;
        }
        Commands::DmsDisable { file_path } => {
            commands::dms_disable::execute(&file_path)?;
        }
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
            dms,
            pqc,
        } => {
            commands::send_file::execute(
                &file_path,
                password.as_deref(),
                remote_path.as_deref(),
                ttl,
                recipients,
                dms,
                pqc,
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
        Commands::SendFileChunked {
            file_path,
            password,
            ttl,
            recipients,
        } => {
            commands::send_file_chunked::execute(&file_path, password.as_deref(), ttl, recipients)?;
        }
        Commands::RecvFileChunked {
            remote_manifest,
            password,
            output,
            recipient,
        } => {
            commands::recv_file_chunked::execute(
                &remote_manifest,
                password.as_deref(),
                output.as_deref(),
                recipient.as_deref(),
            )?;
        }
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
            recursive,
            ttl,
            recipients,
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
