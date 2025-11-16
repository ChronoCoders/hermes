use indicatif::{ProgressBar, ProgressStyle};

pub struct ProgressTracker {
    bar: ProgressBar,
}

impl ProgressTracker {
    #[must_use]
    pub fn new(total: u64, operation: &str) -> Self {
        let bar = ProgressBar::new(total);

        bar.set_style(
            ProgressStyle::default_bar()
                .template(&format!("{{spinner:.green}} {operation} [{{elapsed_precise}}] [{{bar:40.cyan/blue}}] {{bytes}}/{{total_bytes}} ({{eta}})"))
                .unwrap()
                .progress_chars("█▓░░"),
        );

        Self { bar }
    }

    #[must_use]
    pub fn new_spinner(operation: &str) -> Self {
        let bar = ProgressBar::new_spinner();

        bar.set_style(
            ProgressStyle::default_spinner()
                .template(&format!("{{spinner:.green}} {operation} {{msg}}"))
                .unwrap(),
        );

        Self { bar }
    }

    pub fn inc(&self, delta: u64) {
        self.bar.inc(delta);
    }

    pub fn set_position(&self, pos: u64) {
        self.bar.set_position(pos);
    }

    pub fn set_message(&self, msg: String) {
        self.bar.set_message(msg);
    }

    pub fn finish(&self) {
        self.bar.finish();
    }

    pub fn finish_with_message(&self, msg: String) {
        self.bar.finish_with_message(msg);
    }

    pub fn finish_and_clear(&self) {
        self.bar.finish_and_clear();
    }
}

#[must_use]
pub fn create_encryption_progress(size: u64) -> ProgressTracker {
    ProgressTracker::new(size, "🔐 Encrypting")
}

#[must_use]
pub fn create_decryption_progress(size: u64) -> ProgressTracker {
    ProgressTracker::new(size, "🔓 Decrypting")
}

#[must_use]
pub fn create_upload_progress(size: u64) -> ProgressTracker {
    ProgressTracker::new(size, "📤 Uploading")
}

#[must_use]
pub fn create_download_progress(size: u64) -> ProgressTracker {
    ProgressTracker::new(size, "📥 Downloading")
}

#[must_use]
pub fn create_compression_spinner() -> ProgressTracker {
    ProgressTracker::new_spinner("🗜️  Compressing")
}

#[must_use]
pub fn create_keygen_spinner() -> ProgressTracker {
    ProgressTracker::new_spinner("🔑 Generating RSA key")
}
