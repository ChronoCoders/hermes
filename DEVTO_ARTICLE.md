# Building a Post-Quantum Secure File Transfer System in Rust

*How we built Hermes - a military-grade encryption tool that's ready for the quantum computing era*

---

## The Problem That Kept Me Up at Night

Here's something that doesn't get talked about enough: **your encrypted data today could be decrypted tomorrow**.

It's called "harvest now, decrypt later" - adversaries are collecting encrypted communications right now, waiting for quantum computers powerful enough to break RSA and ECC. When that day comes (and cryptographers say it's not *if*, but *when*), all that data becomes readable.

That thought led me to build **Hermes** - a secure file transfer system that's designed to survive the quantum apocalypse.

## What is Hermes?

Hermes is a command-line tool (and now web UI!) for secure file transfer that combines:

- **RSA-4096** (battle-tested classical encryption)
- **Kyber-1024** (NIST-selected post-quantum algorithm)
- **Dilithium-5** (post-quantum digital signatures)
- **AES-256-GCM** (symmetric encryption)
- **Argon2** (memory-hard key derivation)

Think of it as GPG's paranoid cousin who's been reading too many quantum computing papers.

## Why Rust?

Three words: **memory safety matters**.

When you're building cryptographic software, a single buffer overflow or use-after-free bug can compromise everything. Rust's ownership model eliminates entire classes of vulnerabilities at compile time.

Plus, Rust's ecosystem has excellent cryptography crates:
- `rsa` for RSA operations
- `pqc_kyber` for post-quantum key encapsulation
- `pqcrypto-dilithium` for quantum-safe signatures
- `aes-gcm` for authenticated encryption

No wrestling with OpenSSL bindings. No C memory management. Just safe, readable code.

## The Architecture: Defense in Depth

### Hybrid Encryption (Why Not Both?)

Here's our approach: we don't trust any single algorithm completely.

```rust
// Simplified hybrid encryption flow
pub fn encrypt_hybrid(plaintext: &[u8], recipients: Vec<String>) -> Result<Vec<u8>> {
    // 1. Generate random AES key
    let aes_key = generate_random_key();

    // 2. Encrypt data with AES-256-GCM
    let ciphertext = aes_gcm_encrypt(plaintext, &aes_key)?;

    // 3. For each recipient, encrypt AES key with BOTH:
    //    - RSA-4096 (classical security)
    //    - Kyber-1024 (quantum security)
    let mut encrypted_keys = Vec::new();
    for recipient in recipients {
        let rsa_encrypted = rsa_encrypt(&aes_key, &recipient.rsa_pubkey)?;
        let kyber_encrypted = kyber_encapsulate(&aes_key, &recipient.kyber_pubkey)?;
        encrypted_keys.push((rsa_encrypted, kyber_encrypted));
    }

    // 4. Package it all together
    Ok(package_encrypted_data(ciphertext, encrypted_keys))
}
```

If RSA gets broken? Kyber has your back.
If Kyber has an undiscovered flaw? RSA is still there.

This is what cryptographers call "hybrid mode" - and it's exactly what NIST recommends for the transition period.

### Multi-Recipient Encryption

One of my favorite features: encrypt once, share with many.

```bash
# Encrypt for multiple recipients
hermes send-file classified.pdf --recipients alice,bob,charlie --pqc

# Each recipient decrypts with their own key
hermes recv-file classified.pdf.hrms --recipient alice
```

The magic? We encrypt the symmetric key separately for each recipient. The actual file is only encrypted once (efficient!), but each person gets their own "key wrapper" they can open with their private key.

## Features That Make Me Proud

### 1. Steganography Support

Sometimes encryption isn't enough. Sometimes you need *plausible deniability*.

```bash
# Hide encrypted data inside an innocent photo
hermes stego-hide secrets.txt --cover vacation.png --output vacation_final.png -p mypassword
```

We use LSB (Least Significant Bit) steganography - modifying the least important bits of image pixels. The changes are invisible to the human eye, but we can store about 3 bytes of data per 8 pixels.

It's like writing in invisible ink, except the ink is math.

### 2. Key Rotation with Archiving

Security isn't a one-time setup. Keys should be rotated regularly.

```bash
# Rotate keys, archive the old ones
hermes key-rotate alice --archive --pqc --sign
```

This generates fresh keys while preserving the old ones (timestamped in `~/.hermes/keys/archive/`). You can still decrypt old files, but new communications use new keys.

The metadata tracks everything:
```
Last rotated: 2025-11-17T02:15:13Z
RSA fingerprint: f8faa12aab60b171
```

### 3. Shamir's Secret Sharing

What if your private key is so sensitive that no single person should have it?

```bash
# Split key into 5 shares, require any 3 to recover
hermes key-split alice --threshold 3 --shares 5
```

This uses GF(256) polynomial interpolation - the same math that protects Bitcoin multisig wallets. Three executives each get a share. Need to decrypt critical files? Get three of them in a room.

No single point of failure. Beautiful.

### 4. Web UI (New in v2.4.0!)

Not everyone loves the command line. So we built a web interface:

```bash
hermes web-ui --port 8080
```

Open `http://localhost:8080` and you get:
- Key generation and management
- Drag-and-drop file encryption
- Digital signature creation/verification
- Real-time status monitoring

The frontend is a single embedded HTML file (~900 lines of vanilla JS) that talks to an Axum-powered REST API. No npm, no webpack, no React - just fast, minimal dependencies.

## The Hard Parts (Lessons Learned)

### Unicode Box Drawing Characters

Sounds trivial, right? Wrong.

```rust
// This looks innocent...
let header = format!("─[HERMES]─[{title}]─");
let padding = "─".repeat(INNER_WIDTH - header.len());
```

**Bug**: `header.len()` returns *bytes*, not characters. That "─" character is 3 bytes in UTF-8. Our box borders were misaligned for days.

**Fix**:
```rust
let header_char_len = header.chars().count();  // Characters, not bytes!
```

Rust catches many bugs at compile time. This wasn't one of them.

### ANSI Color Codes in Terminal Width Calculations

When you colorize terminal output, you add invisible escape sequences:

```
\x1b[36m│\x1b[0m This looks like 2 chars, but it's actually 11 bytes
```

We had to write a helper to strip ANSI codes before calculating padding:

```rust
fn strip_ansi_codes(s: &str) -> String {
    let mut result = String::new();
    let mut in_escape = false;

    for c in s.chars() {
        if c == '\x1b' {
            in_escape = true;
        } else if in_escape && c == 'm' {
            in_escape = false;
        } else if !in_escape {
            result.push(c);
        }
    }
    result
}
```

### Post-Quantum Key Sizes

Kyber public keys are about 1,568 bytes. Dilithium public keys? 2,592 bytes. RSA-4096 public keys are around 550 bytes.

Suddenly your encrypted packages are *much* bigger. Worth it for quantum security? Absolutely. But it required redesigning our package format and increasing size limits across the board.

## The Package Format (v0x02)

Every encrypted file follows this structure:

```
[MAGIC: 4 bytes]    "HRMS"
[VERSION: 1 byte]   0x02
[FLAGS: 1 byte]     bit 2 = PQC enabled
[SALT: 16 bytes]    For key derivation
[NONCE: 12 bytes]   For AES-GCM
[RECIPIENT_COUNT]   How many can decrypt
[RECIPIENT_DATA]    Encrypted keys per recipient
[CIPHERTEXT]        The actual encrypted data
[TAG: 16 bytes]     Authentication tag
```

Everything is versioned. We can add features without breaking old files.

## Performance Numbers

On my machine (Ryzen 9 5950X):

- **Key Generation** (RSA-4096 + Kyber + Dilithium): ~3 seconds
- **File Encryption** (1MB, password-based): ~50ms
- **Hybrid Encryption** (1MB, 3 recipients, PQC): ~200ms
- **Steganography Embed** (1KB in 1080p image): ~100ms

Not bad for paranoia-level security.

## What's Next?

Ideas brewing for future versions:

1. **Hardware Security Module (HSM) support** - Keep private keys in secure hardware
2. **Tor integration** - Anonymous file drops
3. **Mobile apps** - Because security shouldn't require a laptop
4. **Audit logging** - Cryptographic proof of who decrypted what, when
5. **Zero-knowledge proofs** - Prove you have access without revealing the key

## Try It Yourself

```bash
# Clone the repo
git clone https://github.com/ChronoCoders/hermes.git
cd hermes

# Build it
cargo build --release

# Generate your first quantum-safe keypair
./target/release/hermes keygen mykey --pqc --sign

# Encrypt a message
echo "Hello, quantum-safe world!" | ./target/release/hermes send-msg -p secretpassword

# Start the web UI
./target/release/hermes web-ui
```

The full source is on GitHub with extensive documentation, release notes for each version, and a detailed changelog.

## Final Thoughts

Building Hermes taught me that security is never "done." It's layers upon layers of defense, constant vigilance, and planning for threats that don't exist yet.

Is Hermes perfect? No. Will quantum computers break everything tomorrow? Probably not. But when they do arrive, and they will, having tools that are ready feels like the responsible thing to do.

The best time to prepare for quantum computing was yesterday. The second best time is now.

---

*Hermes is open-source and MIT licensed. Star it on GitHub, report issues, contribute features. Let's make secure communication accessible to everyone.*

**Tech Stack**: Rust, Axum, Tokio, CRYSTALS-Kyber, CRYSTALS-Dilithium, RSA, AES-256-GCM, Argon2

**Current Version**: 2.4.0

**Tags**: #rust #cryptography #security #postquantum #opensource

---

*What security challenges are you tackling? Have you started thinking about post-quantum cryptography? Drop a comment below - I'd love to hear your thoughts!*
