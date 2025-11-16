use crate::config::Settings;
use crate::error::{HermesError, Result};
use crate::ui;
use ssh2::Session;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::path::Path;

pub struct SftpClient {
    pub session: Session,
}

impl SftpClient {
    pub fn connect(config: &Settings) -> Result<Self> {
        let spinner = ui::create_spinner("Connecting to SFTP server...");
        spinner.enable_steady_tick(std::time::Duration::from_millis(100));

        let tcp = TcpStream::connect(format!("{}:{}", config.sftp.host, config.sftp.port))
            .map_err(|e| HermesError::SftpConnectionFailed(format!("Connection failed: {e}")))?;

        let mut session = Session::new().map_err(|e| {
            HermesError::SftpConnectionFailed(format!("Session creation failed: {e}"))
        })?;

        session.set_tcp_stream(tcp);
        session
            .handshake()
            .map_err(|e| HermesError::SftpConnectionFailed(format!("Handshake failed: {e}")))?;

        if let Some(key_file) = &config.sftp.key_file {
            session
                .userauth_pubkey_file(&config.sftp.username, None, Path::new(key_file), None)
                .map_err(|e| {
                    HermesError::SftpConnectionFailed(format!("Key authentication failed: {e}"))
                })?;
        } else {
            return Err(HermesError::SftpConnectionFailed(
                "No key file specified".to_string(),
            ));
        }

        spinner.finish_and_clear();

        Ok(Self { session })
    }

    pub fn upload(&self, data: &[u8], remote_path: &str) -> Result<()> {
        let sftp = self
            .session
            .sftp()
            .map_err(|e| HermesError::SftpOperationFailed(format!("SFTP init failed: {e}")))?;

        let pb = ui::create_progress_bar(data.len() as u64);
        pb.set_message("Uploading...");

        let mut remote_file = sftp
            .create(Path::new(remote_path))
            .map_err(|e| HermesError::SftpOperationFailed(format!("File creation failed: {e}")))?;

        let chunk_size = 8192;
        for (i, chunk) in data.chunks(chunk_size).enumerate() {
            remote_file
                .write_all(chunk)
                .map_err(|e| HermesError::SftpOperationFailed(format!("Upload failed: {e}")))?;
            pb.set_position(((i + 1) * chunk_size).min(data.len()) as u64);
        }

        pb.finish_and_clear();

        Ok(())
    }

    pub fn download(&self, remote_path: &str) -> Result<Vec<u8>> {
        let sftp = self
            .session
            .sftp()
            .map_err(|e| HermesError::SftpOperationFailed(format!("SFTP init failed: {e}")))?;

        let mut remote_file = sftp
            .open(Path::new(remote_path))
            .map_err(|e| HermesError::SftpOperationFailed(format!("File not found: {e}")))?;

        let stat = remote_file
            .stat()
            .map_err(|e| HermesError::SftpOperationFailed(format!("Stat failed: {e}")))?;

        let pb = ui::create_progress_bar(stat.size.unwrap_or(0));
        pb.set_message("Downloading...");

        let mut buffer = Vec::new();
        let mut temp_buf = [0u8; 8192];

        loop {
            match remote_file.read(&mut temp_buf) {
                Ok(0) => break,
                Ok(n) => {
                    buffer.extend_from_slice(&temp_buf[..n]);
                    pb.set_position(buffer.len() as u64);
                }
                Err(e) => {
                    return Err(HermesError::SftpOperationFailed(format!(
                        "Download failed: {e}"
                    )))
                }
            }
        }

        pb.finish_and_clear();

        Ok(buffer)
    }
}
