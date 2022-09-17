mod lib;
// I added the 'mode lib' via intelli-j rust as without it I couldn't fully navigate all symbols in lib.rs
// Related to: https://github.com/intellij-rust/intellij-rust/issues/5488
// https://github.com/intellij-rust/intellij-rust/issues/2389#event-7407691313

use clap::Parser;

fn main() {
    let config = lib::Config::parse();
    if let Err(e) = lib::run(config) {
        eprintln!("{}", e);
        std::process::exit(1);
    }

    //let config = headr::Config::parse();
    //match headr::run(config) {
    //if let Err(e) = headr::run(config) {
    //    eprintln!("{}", e);
    //    std::process::exit(1);
    // }
}
