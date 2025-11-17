use axum::{extract::Json, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

use crate::crypto;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    success: bool,
    data: Option<T>,
    error: Option<String>,
}

impl<T: Serialize> ApiResponse<T> {
    fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    fn error(msg: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(msg),
        }
    }
}

#[derive(Serialize)]
pub struct StatusInfo {
    version: String,
    initialized: bool,
    keys_count: usize,
    recipients_count: usize,
}

#[derive(Serialize)]
pub struct KeyInfo {
    name: String,
    key_type: String,
    fingerprint: String,
    has_pqc: bool,
    has_signing: bool,
}

#[derive(Deserialize)]
pub struct GenerateKeyRequest {
    name: String,
    pqc: bool,
    sign: bool,
}

#[derive(Deserialize)]
pub struct RotateKeyRequest {
    name: String,
    archive: bool,
    pqc: bool,
    sign: bool,
}

#[derive(Deserialize)]
pub struct EncryptMessageRequest {
    message: String,
    password: Option<String>,
    recipients: Option<Vec<String>>,
}

#[derive(Serialize)]
pub struct EncryptMessageResponse {
    encrypted: String,
    size: usize,
}

#[derive(Deserialize)]
pub struct DecryptMessageRequest {
    encrypted: String,
    password: Option<String>,
    recipient: Option<String>,
}

#[derive(Serialize)]
pub struct DecryptMessageResponse {
    message: String,
}

#[derive(Deserialize)]
pub struct EncryptFileRequest {
    #[allow(dead_code)]
    filename: String,
    data: String, // base64 encoded
    password: Option<String>,
    recipients: Option<Vec<String>>,
    pqc: bool,
}

#[derive(Serialize)]
pub struct EncryptFileResponse {
    encrypted: String, // base64 encoded
    size: usize,
}

#[derive(Deserialize)]
pub struct DecryptFileRequest {
    encrypted: String, // base64 encoded
    password: Option<String>,
    recipient: Option<String>,
}

#[derive(Serialize)]
pub struct DecryptFileResponse {
    data: String, // base64 encoded
    size: usize,
}

#[derive(Deserialize)]
pub struct SignRequest {
    data: String, // base64 encoded
    key_name: String,
}

#[derive(Serialize)]
pub struct SignResponse {
    signed: String, // base64 encoded
}

#[derive(Deserialize)]
pub struct VerifyRequest {
    signed: String, // base64 encoded
    signer: String,
}

#[derive(Serialize)]
pub struct VerifyResponse {
    valid: bool,
    data: String, // base64 encoded
}

#[derive(Deserialize)]
pub struct StegoCapacityRequest {
    image_data: String, // base64 encoded PNG
}

#[derive(Serialize)]
pub struct StegoCapacityResponse {
    capacity_bytes: usize,
    width: u32,
    height: u32,
}

#[derive(Serialize)]
pub struct ConfigInfo {
    home_dir: String,
    keys_dir: String,
    recipients_dir: String,
    config_file: String,
}

#[derive(Serialize)]
pub struct ArchivedKeyInfo {
    archive_id: String,
    files: Vec<String>,
}

// API Handlers

pub async fn status() -> impl IntoResponse {
    let home_dir = match dirs::home_dir() {
        Some(dir) => dir.join(".hermes"),
        None => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<StatusInfo>::error(
                    "Could not find home directory".to_string(),
                )),
            )
        }
    };

    let initialized = home_dir.join("config.toml").exists();

    let keys_count = if home_dir.join("keys").exists() {
        fs::read_dir(home_dir.join("keys"))
            .map(|entries| {
                entries
                    .filter_map(|e| e.ok())
                    .filter(|e| {
                        e.path()
                            .extension()
                            .map(|ext| ext == "pub")
                            .unwrap_or(false)
                    })
                    .count()
            })
            .unwrap_or(0)
    } else {
        0
    };

    let recipients_count = if home_dir.join("recipients").exists() {
        fs::read_dir(home_dir.join("recipients"))
            .map(|entries| {
                entries
                    .filter_map(|e| e.ok())
                    .filter(|e| {
                        e.path()
                            .extension()
                            .map(|ext| ext == "pub")
                            .unwrap_or(false)
                    })
                    .count()
            })
            .unwrap_or(0)
    } else {
        0
    };

    let info = StatusInfo {
        version: env!("CARGO_PKG_VERSION").to_string(),
        initialized,
        keys_count,
        recipients_count,
    };

    (StatusCode::OK, Json(ApiResponse::success(info)))
}

pub async fn list_keys() -> impl IntoResponse {
    let keys_dir = match dirs::home_dir() {
        Some(dir) => dir.join(".hermes").join("keys"),
        None => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<Vec<KeyInfo>>::error(
                    "Could not find home directory".to_string(),
                )),
            )
        }
    };

    if !keys_dir.exists() {
        return (
            StatusCode::OK,
            Json(ApiResponse::success(Vec::<KeyInfo>::new())),
        );
    }

    let mut keys = Vec::new();
    let entries = match fs::read_dir(&keys_dir) {
        Ok(e) => e,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<Vec<KeyInfo>>::error(e.to_string())),
            )
        }
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().map(|e| e == "pub").unwrap_or(false) {
            let filename = path.file_stem().unwrap().to_string_lossy().to_string();

            // Skip Kyber and Dilithium keys (they're suffixes)
            if filename.ends_with("_kyber") || filename.ends_with("_dilithium") {
                continue;
            }

            let has_pqc = keys_dir.join(format!("{}_kyber.pub", filename)).exists();
            let has_signing = keys_dir.join(format!("{}_dilithium.pub", filename)).exists();

            let fingerprint = match crypto::load_public_key(path.to_str().unwrap()) {
                Ok(key) => crypto::get_key_fingerprint(&key).unwrap_or_default(),
                Err(_) => "unknown".to_string(),
            };

            keys.push(KeyInfo {
                name: filename,
                key_type: "RSA-4096".to_string(),
                fingerprint,
                has_pqc,
                has_signing,
            });
        }
    }

    (StatusCode::OK, Json(ApiResponse::success(keys)))
}

pub async fn generate_key(Json(req): Json<GenerateKeyRequest>) -> impl IntoResponse {
    match crate::commands::keygen::execute(&req.name, None, req.pqc, req.sign) {
        Ok(_) => (
            StatusCode::OK,
            Json(ApiResponse::success("Key generated successfully")),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::<&str>::error(e.to_string())),
        ),
    }
}

pub async fn rotate_key(Json(req): Json<RotateKeyRequest>) -> impl IntoResponse {
    match crate::commands::key_rotate::execute(&req.name, req.archive, req.pqc, req.sign) {
        Ok(_) => (
            StatusCode::OK,
            Json(ApiResponse::success("Key rotated successfully")),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::<&str>::error(e.to_string())),
        ),
    }
}

pub async fn list_archived_keys() -> impl IntoResponse {
    let archive_dir = match dirs::home_dir() {
        Some(dir) => dir.join(".hermes").join("keys").join("archive"),
        None => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<Vec<ArchivedKeyInfo>>::error(
                    "Could not find home directory".to_string(),
                )),
            )
        }
    };

    if !archive_dir.exists() {
        return (
            StatusCode::OK,
            Json(ApiResponse::success(Vec::<ArchivedKeyInfo>::new())),
        );
    }

    let entries = match fs::read_dir(&archive_dir) {
        Ok(e) => e,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<Vec<ArchivedKeyInfo>>::error(e.to_string())),
            )
        }
    };

    let mut archives: HashMap<String, Vec<String>> = HashMap::new();

    for entry in entries.flatten() {
        let filename = entry.file_name().to_string_lossy().to_string();
        if filename.ends_with(".pem") || filename.ends_with(".pub") {
            if let Some(base) = filename.rsplit('.').nth(1) {
                if let Some(pos) = base.rfind('_') {
                    let parts: Vec<&str> = base.splitn(2, '_').collect();
                    if parts.len() >= 2 {
                        let key_name = parts[0];
                        let timestamp = &base[pos + 1..];
                        let archive_id = format!("{}_{}", key_name, timestamp);
                        archives.entry(archive_id).or_default().push(filename);
                    }
                }
            }
        }
    }

    let mut result: Vec<ArchivedKeyInfo> = archives
        .into_iter()
        .map(|(id, files)| ArchivedKeyInfo {
            archive_id: id,
            files,
        })
        .collect();
    result.sort_by(|a, b| b.archive_id.cmp(&a.archive_id));

    (StatusCode::OK, Json(ApiResponse::success(result)))
}

pub async fn encrypt_message(Json(req): Json<EncryptMessageRequest>) -> impl IntoResponse {
    use crate::crypto::encrypt::{encrypt_data, encrypt_data_multi};

    let encrypted = if let Some(pwd) = req.password {
        match encrypt_data(req.message.as_bytes(), &pwd, None, None) {
            Ok(data) => data,
            Err(e) => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(ApiResponse::<EncryptMessageResponse>::error(e.to_string())),
                )
            }
        }
    } else if let Some(recipients) = req.recipients {
        match encrypt_data_multi(req.message.as_bytes(), None, None, None, Some(recipients), false)
        {
            Ok(data) => data,
            Err(e) => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(ApiResponse::<EncryptMessageResponse>::error(e.to_string())),
                )
            }
        }
    } else {
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::<EncryptMessageResponse>::error(
                "Must provide password or recipients".to_string(),
            )),
        );
    };

    let size = encrypted.len();
    let encoded = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &encrypted);

    (
        StatusCode::OK,
        Json(ApiResponse::success(EncryptMessageResponse {
            encrypted: encoded,
            size,
        })),
    )
}

pub async fn decrypt_message(Json(req): Json<DecryptMessageRequest>) -> impl IntoResponse {
    use crate::crypto::decrypt::{decrypt_data, decrypt_data_multi};

    let encrypted = match base64::Engine::decode(
        &base64::engine::general_purpose::STANDARD,
        &req.encrypted,
    ) {
        Ok(data) => data,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(ApiResponse::<DecryptMessageResponse>::error(e.to_string())),
            )
        }
    };

    let decrypted = if let Some(pwd) = req.password {
        match decrypt_data(&encrypted, &pwd) {
            Ok(data) => data,
            Err(e) => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(ApiResponse::<DecryptMessageResponse>::error(e.to_string())),
                )
            }
        }
    } else if let Some(recipient) = req.recipient {
        match decrypt_data_multi(&encrypted, &recipient) {
            Ok(data) => data,
            Err(e) => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(ApiResponse::<DecryptMessageResponse>::error(e.to_string())),
                )
            }
        }
    } else {
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::<DecryptMessageResponse>::error(
                "Must provide password or recipient".to_string(),
            )),
        );
    };

    let message = match String::from_utf8(decrypted) {
        Ok(s) => s,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(ApiResponse::<DecryptMessageResponse>::error(e.to_string())),
            )
        }
    };

    (
        StatusCode::OK,
        Json(ApiResponse::success(DecryptMessageResponse { message })),
    )
}

pub async fn encrypt_file(Json(req): Json<EncryptFileRequest>) -> impl IntoResponse {
    use crate::crypto::encrypt::{encrypt_data, encrypt_data_multi};

    let file_data = match base64::Engine::decode(
        &base64::engine::general_purpose::STANDARD,
        &req.data,
    ) {
        Ok(data) => data,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(ApiResponse::<EncryptFileResponse>::error(e.to_string())),
            )
        }
    };

    let encrypted = if let Some(pwd) = req.password {
        match encrypt_data(&file_data, &pwd, None, None) {
            Ok(data) => data,
            Err(e) => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(ApiResponse::<EncryptFileResponse>::error(e.to_string())),
                )
            }
        }
    } else if let Some(recipients) = req.recipients {
        match encrypt_data_multi(&file_data, None, None, None, Some(recipients), req.pqc) {
            Ok(data) => data,
            Err(e) => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(ApiResponse::<EncryptFileResponse>::error(e.to_string())),
                )
            }
        }
    } else {
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::<EncryptFileResponse>::error(
                "Must provide password or recipients".to_string(),
            )),
        );
    };

    let size = encrypted.len();
    let encoded = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &encrypted);

    (
        StatusCode::OK,
        Json(ApiResponse::success(EncryptFileResponse {
            encrypted: encoded,
            size,
        })),
    )
}

pub async fn decrypt_file(Json(req): Json<DecryptFileRequest>) -> impl IntoResponse {
    use crate::crypto::decrypt::{decrypt_data, decrypt_data_multi};

    let encrypted = match base64::Engine::decode(
        &base64::engine::general_purpose::STANDARD,
        &req.encrypted,
    ) {
        Ok(data) => data,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(ApiResponse::<DecryptFileResponse>::error(e.to_string())),
            )
        }
    };

    let decrypted = if let Some(pwd) = req.password {
        match decrypt_data(&encrypted, &pwd) {
            Ok(data) => data,
            Err(e) => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(ApiResponse::<DecryptFileResponse>::error(e.to_string())),
                )
            }
        }
    } else if let Some(recipient) = req.recipient {
        match decrypt_data_multi(&encrypted, &recipient) {
            Ok(data) => data,
            Err(e) => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(ApiResponse::<DecryptFileResponse>::error(e.to_string())),
                )
            }
        }
    } else {
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::<DecryptFileResponse>::error(
                "Must provide password or recipient".to_string(),
            )),
        );
    };

    let size = decrypted.len();
    let encoded = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &decrypted);

    (
        StatusCode::OK,
        Json(ApiResponse::success(DecryptFileResponse {
            data: encoded,
            size,
        })),
    )
}

pub async fn sign_data(Json(req): Json<SignRequest>) -> impl IntoResponse {
    let data =
        match base64::Engine::decode(&base64::engine::general_purpose::STANDARD, &req.data) {
            Ok(d) => d,
            Err(e) => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(ApiResponse::<SignResponse>::error(e.to_string())),
                )
            }
        };

    let keys_dir = match dirs::home_dir() {
        Some(dir) => dir.join(".hermes").join("keys"),
        None => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<SignResponse>::error(
                    "Could not find home directory".to_string(),
                )),
            )
        }
    };

    let key_path = keys_dir.join(format!("{}_dilithium.pem", req.key_name));
    if !key_path.exists() {
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::<SignResponse>::error(format!(
                "Dilithium key not found for: {}",
                req.key_name
            ))),
        );
    }

    let secret_key = match crypto::load_dilithium_secret_key(key_path.to_str().unwrap()) {
        Ok(k) => k,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<SignResponse>::error(e.to_string())),
            )
        }
    };

    let signed = crypto::sign_message(&data, &secret_key);
    let encoded = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &signed);

    (
        StatusCode::OK,
        Json(ApiResponse::success(SignResponse { signed: encoded })),
    )
}

pub async fn verify_signature(Json(req): Json<VerifyRequest>) -> impl IntoResponse {
    let signed =
        match base64::Engine::decode(&base64::engine::general_purpose::STANDARD, &req.signed) {
            Ok(d) => d,
            Err(e) => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(ApiResponse::<VerifyResponse>::error(e.to_string())),
                )
            }
        };

    let recipients_dir = match dirs::home_dir() {
        Some(dir) => dir.join(".hermes").join("recipients"),
        None => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<VerifyResponse>::error(
                    "Could not find home directory".to_string(),
                )),
            )
        }
    };

    let keys_dir = match dirs::home_dir() {
        Some(dir) => dir.join(".hermes").join("keys"),
        None => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<VerifyResponse>::error(
                    "Could not find home directory".to_string(),
                )),
            )
        }
    };

    let key_path = if recipients_dir
        .join(format!("{}_dilithium.pub", req.signer))
        .exists()
    {
        recipients_dir.join(format!("{}_dilithium.pub", req.signer))
    } else if keys_dir
        .join(format!("{}_dilithium.pub", req.signer))
        .exists()
    {
        keys_dir.join(format!("{}_dilithium.pub", req.signer))
    } else {
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::<VerifyResponse>::error(format!(
                "Dilithium public key not found for: {}",
                req.signer
            ))),
        );
    };

    let public_key = match crypto::load_dilithium_public_key(key_path.to_str().unwrap()) {
        Ok(k) => k,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<VerifyResponse>::error(e.to_string())),
            )
        }
    };

    match crypto::verify_signature(&signed, &public_key) {
        Ok(original) => {
            let encoded =
                base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &original);
            (
                StatusCode::OK,
                Json(ApiResponse::success(VerifyResponse {
                    valid: true,
                    data: encoded,
                })),
            )
        }
        Err(_) => (
            StatusCode::OK,
            Json(ApiResponse::success(VerifyResponse {
                valid: false,
                data: String::new(),
            })),
        ),
    }
}

pub async fn stego_capacity(Json(req): Json<StegoCapacityRequest>) -> impl IntoResponse {
    let image_data = match base64::Engine::decode(
        &base64::engine::general_purpose::STANDARD,
        &req.image_data,
    ) {
        Ok(d) => d,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(ApiResponse::<StegoCapacityResponse>::error(e.to_string())),
            )
        }
    };

    let img = match image::load_from_memory(&image_data) {
        Ok(i) => i,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(ApiResponse::<StegoCapacityResponse>::error(e.to_string())),
            )
        }
    };

    let (width, height) = (img.width(), img.height());
    let capacity = ((width * height * 3) / 8) as usize - 12; // subtract header size

    (
        StatusCode::OK,
        Json(ApiResponse::success(StegoCapacityResponse {
            capacity_bytes: capacity,
            width,
            height,
        })),
    )
}

pub async fn get_config() -> impl IntoResponse {
    let home = match dirs::home_dir() {
        Some(dir) => dir.join(".hermes"),
        None => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<ConfigInfo>::error(
                    "Could not find home directory".to_string(),
                )),
            )
        }
    };

    (
        StatusCode::OK,
        Json(ApiResponse::success(ConfigInfo {
            home_dir: home.to_string_lossy().to_string(),
            keys_dir: home.join("keys").to_string_lossy().to_string(),
            recipients_dir: home.join("recipients").to_string_lossy().to_string(),
            config_file: home.join("config.toml").to_string_lossy().to_string(),
        })),
    )
}
