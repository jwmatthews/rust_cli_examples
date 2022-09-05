use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)] // Read from `Cargo.toml`
struct Cli {
    /// Text to echo
    #[clap(required = true, min_values = 1)]
    text: Vec<String>,

    #[clap(short = 'n', long = "omit_newlane")]
    omit_newline: bool,
}

fn main() {
    let cli = Cli::parse();
    let text = cli.text;
    let omit_newline = cli.omit_newline;

    let ending = if omit_newline { "" } else { "\n" };
    print!("{}{}", text.join(" "), ending);
}
