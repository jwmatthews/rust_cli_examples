use clap::Parser;

fn main() {
    let config = catr::Config::parse();
    if let Err(e) = catr::run(config) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
