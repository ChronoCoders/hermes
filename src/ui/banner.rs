use colored::Colorize;

pub fn print_banner() {
    println!(
        "{}",
        "╔═══════════════════════════════════════════════════════════╗".cyan()
    );
    println!(
        "{}",
        "║                                                           ║".cyan()
    );
    println!(
        "{}",
        "║    ██╗  ██╗███████╗██████╗ ███╗   ███╗███████╗███████╗    ║".cyan()
    );
    println!(
        "{}",
        "║    ██║  ██║██╔════╝██╔══██╗████╗ ████║██╔════╝██╔════╝    ║".cyan()
    );
    println!(
        "{}",
        "║    ███████║█████╗  ██████╔╝██╔████╔██║█████╗  ███████╗    ║".cyan()
    );
    println!(
        "{}",
        "║    ██╔══██║██╔══╝  ██╔══██╗██║╚██╔╝██║██╔══╝  ╚════██║    ║".cyan()
    );
    println!(
        "{}",
        "║    ██║  ██║███████╗██║  ██║██║ ╚═╝ ██║███████╗███████║    ║".cyan()
    );
    println!(
        "{}",
        "║    ╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝╚═╝     ╚═╝╚══════╝╚══════╝    ║".cyan()
    );
    println!(
        "{}",
        "║                                                           ║".cyan()
    );
    println!(
        "{}",
        "║         SECURE TRANSFER PROTOCOL v1.0 [ENCRYPTED]         ║".cyan()
    );
    println!(
        "{}",
        "║         MILITARY-GRADE • AES-256-GCM • ARGON2             ║".cyan()
    );
    println!(
        "{}",
        "╚═══════════════════════════════════════════════════════════╝".cyan()
    );
    println!();
    println!("{}", "[SYSTEM READY] Awaiting command...".bright_green());
    println!();
}

pub fn print_box_start(title: &str) {
    println!(
        "{}",
        format!("┌─[HERMES]─[{title}]─────────────────────────────────────┐").cyan()
    );
}

pub fn print_box_line(content: &str) {
    println!("{} {} {}", "│".cyan(), content, "│".cyan());
}

pub fn print_box_end() {
    println!(
        "{}",
        "└─────────────────────────────────────────────────────────┘".cyan()
    );
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
