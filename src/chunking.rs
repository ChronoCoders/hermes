use crate::error::{HermesError, Result};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

const CHUNK_SIZE: usize = 50 * 1024 * 1024;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChunkManifest {
    pub original_filename: String,
    pub total_size: u64,
    pub chunk_size: usize,
    pub total_chunks: usize,
    pub file_hash: String,
    pub chunks: Vec<ChunkInfo>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChunkInfo {
    pub index: usize,
    pub size: u64,
    pub hash: String,
    pub encrypted_path: String,
}

impl ChunkManifest {
    #[must_use]
    pub fn new(filename: String, total_size: u64) -> Self {
        let total_chunks = ((total_size as f64) / (CHUNK_SIZE as f64)).ceil() as usize;

        Self {
            original_filename: filename,
            total_size,
            chunk_size: CHUNK_SIZE,
            total_chunks,
            file_hash: String::new(),
            chunks: Vec::new(),
        }
    }

    pub fn add_chunk(&mut self, chunk: ChunkInfo) {
        self.chunks.push(chunk);
    }

    pub fn set_file_hash(&mut self, hash: String) {
        self.file_hash = hash;
    }

    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string_pretty(self).map_err(HermesError::SerializationError)
    }

    pub fn from_json(json: &str) -> Result<Self> {
        serde_json::from_str(json).map_err(HermesError::SerializationError)
    }

    pub fn save_to_file(&self, path: &Path) -> Result<()> {
        let json = self.to_json()?;
        let mut file = File::create(path)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    pub fn load_from_file(path: &Path) -> Result<Self> {
        let mut file = File::open(path)?;
        let mut json = String::new();
        file.read_to_string(&mut json)?;
        Self::from_json(&json)
    }

    pub fn verify_integrity(&self) -> Result<()> {
        if self.chunks.len() != self.total_chunks {
            return Err(HermesError::ConfigError(format!(
                "Expected {} chunks, found {}",
                self.total_chunks,
                self.chunks.len()
            )));
        }

        for (i, chunk) in self.chunks.iter().enumerate() {
            if chunk.index != i {
                return Err(HermesError::ConfigError(format!(
                    "Chunk index mismatch at position {}: expected {}, found {}",
                    i, i, chunk.index
                )));
            }
        }

        Ok(())
    }
}

pub fn split_file_into_chunks<P: AsRef<Path>>(
    file_path: P,
    output_dir: P,
) -> Result<ChunkManifest> {
    let file_path = file_path.as_ref();
    let output_dir = output_dir.as_ref();

    let filename = file_path
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| HermesError::FileNotFound("Invalid filename".to_string()))?
        .to_string();

    let mut file = File::open(file_path)?;
    let file_size = file.metadata()?.len();

    let mut manifest = ChunkManifest::new(filename.clone(), file_size);
    let mut file_hasher = Sha256::new();

    let mut chunk_index = 0;
    let mut buffer = vec![0u8; CHUNK_SIZE];

    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }

        file_hasher.update(&buffer[..bytes_read]);

        let mut chunk_hasher = Sha256::new();
        chunk_hasher.update(&buffer[..bytes_read]);
        let chunk_hash = format!("{:x}", chunk_hasher.finalize());

        let chunk_filename = format!("{}.chunk.{:03}", filename, chunk_index + 1);
        let chunk_path = output_dir.join(&chunk_filename);

        let mut chunk_file = File::create(&chunk_path)?;
        chunk_file.write_all(&buffer[..bytes_read])?;

        let chunk_info = ChunkInfo {
            index: chunk_index,
            size: bytes_read as u64,
            hash: chunk_hash,
            encrypted_path: chunk_filename,
        };

        manifest.add_chunk(chunk_info);
        chunk_index += 1;
    }

    let file_hash = format!("{:x}", file_hasher.finalize());
    manifest.set_file_hash(file_hash);

    Ok(manifest)
}

pub fn reassemble_chunks_from_manifest<P: AsRef<Path>>(
    manifest: &ChunkManifest,
    chunks_dir: P,
    output_path: P,
) -> Result<()> {
    let chunks_dir = chunks_dir.as_ref();
    let output_path = output_path.as_ref();

    manifest.verify_integrity()?;

    let mut output_file = File::create(output_path)?;
    let mut file_hasher = Sha256::new();

    for chunk in &manifest.chunks {
        let chunk_path = chunks_dir.join(&chunk.encrypted_path);

        if !chunk_path.exists() {
            return Err(HermesError::FileNotFound(format!(
                "Chunk file not found: {}",
                chunk.encrypted_path
            )));
        }

        let mut chunk_file = File::open(&chunk_path)?;
        let mut chunk_data = Vec::new();
        chunk_file.read_to_end(&mut chunk_data)?;

        let mut chunk_hasher = Sha256::new();
        chunk_hasher.update(&chunk_data);
        let calculated_hash = format!("{:x}", chunk_hasher.finalize());

        if calculated_hash != chunk.hash {
            return Err(HermesError::ConfigError(format!(
                "Chunk {} hash mismatch: expected {}, got {}",
                chunk.index, chunk.hash, calculated_hash
            )));
        }

        file_hasher.update(&chunk_data);
        output_file.write_all(&chunk_data)?;
    }

    let calculated_file_hash = format!("{:x}", file_hasher.finalize());
    if calculated_file_hash != manifest.file_hash {
        return Err(HermesError::ConfigError(format!(
            "File hash mismatch: expected {}, got {}",
            manifest.file_hash, calculated_file_hash
        )));
    }

    Ok(())
}

pub fn calculate_file_hash<P: AsRef<Path>>(path: P) -> Result<String> {
    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buffer = vec![0u8; 8192];

    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    Ok(format!("{:x}", hasher.finalize()))
}
