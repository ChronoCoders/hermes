use colored::Colorize;

const BOX_WIDTH: usize = 61;
const INNER_WIDTH: usize = BOX_WIDTH - 2; // Subtract 2 for border chars

pub fn print_banner() {
    let version = env!("CARGO_PKG_VERSION");
    let version_line = format!("SECURE TRANSFER PROTOCOL v{version} [ENCRYPTED]");
    let features_line = "PQC-HYBRID • AES-256-GCM • DILITHIUM • KYBER";

    println!("{}", "╔═══════════════════════════════════════════════════════════╗".cyan());
    println!("{}", "║                                                           ║".cyan());
    println!("{}", "║    ██╗  ██╗███████╗██████╗ ███╗   ███╗███████╗███████╗   ║".cyan());
    println!("{}", "║    ██║  ██║██╔════╝██╔══██╗████╗ ████║██╔════╝██╔════╝   ║".cyan());
    println!("{}", "║    ███████║█████╗  ██████╔╝██╔████╔██║█████╗  ███████╗   ║".cyan());
    println!("{}", "║    ██╔══██║██╔══╝  ██╔══██╗██║╚██╔╝██║██╔══╝  ╚════██║   ║".cyan());
    println!("{}", "║    ██║  ██║███████╗██║  ██║██║ ╚═╝ ██║███████╗███████║   ║".cyan());
    println!("{}", "║    ╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝╚═╝     ╚═╝╚══════╝╚══════╝   ║".cyan());
    println!("{}", "║                                                           ║".cyan());
    println!("{}", format!("║{:^59}║", version_line).cyan());
    println!("{}", format!("║{:^59}║", features_line).cyan());
    println!("{}", "╚═══════════════════════════════════════════════════════════╝".cyan());
    println!();
    println!("{}", "[SYSTEM READY] Awaiting command...".bright_green());
    println!();
}

pub fn print_box_start(title: &str) {
    let header = format!("─[HERMES]─[{title}]─");
    let header_char_len = header.chars().count();
    let padding_len = INNER_WIDTH.saturating_sub(header_char_len);
    let padding = "─".repeat(padding_len);
    println!("{}", format!("┌{header}{padding}┐").cyan());
}

pub fn print_box_line(content: &str) {
    // Calculate visible length (without ANSI codes)
    let visible_len = strip_ansi_codes(content).chars().count();
    let padding_len = INNER_WIDTH.saturating_sub(visible_len);
    let padding = " ".repeat(padding_len);
    println!("{}{}{}{}", "│".cyan(), content, padding, "│".cyan());
}

pub fn print_box_end() {
    let bottom = "─".repeat(INNER_WIDTH);
    println!("{}", format!("└{bottom}┘").cyan());
}

pub fn print_success(message: &str) {
    println!("{}", format!("✓ {message}").bright_green().bold());
}

pub fn print_error(message: &str) {
    println!("{}", format!("✗ {message}").bright_red().bold());
}

pub fn print_info(label: &str, value: &str) {
    println!("  {}: {}", label.bright_white(), value.bright_magenta());
}

pub fn print_status(status: &str) {
    println!("  Status: {}", format!("[{status}]").bright_green());
}

fn strip_ansi_codes(s: &str) -> String {
    let mut result = String::new();
    let mut in_escape = false;

    for c in s.chars() {
        if c == '\x1b' {
            in_escape = true;
        } else if in_escape {
            if c == 'm' {
                in_escape = false;
            }
        } else {
            result.push(c);
        }
    }

    result
}
