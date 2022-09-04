# rust_cli_examples
Learning repo for playing with Rust by reimplementing common Unix CLI examples

Heavily inspired and borrowing from: 
* [Command-Line Rust](https://www.oreilly.com/library/view/command-line-rust/9781098109424/) by Ken Youens-Clark
  * https://github.com/kyclark/command-line-rust
* [Command line apps in Rust](https://rust-cli.github.io/book/index.html)
* 

# Notes on how I'm approaching learning Rust
1. I started with: [Programming Rust, 2nd edition](https://www.oreilly.com/library/view/programming-rust-2nd/9781492052586/) by Jim Blandy, Jason Orendorff, Leonora F. S. Tindall
    1. My mind didn't fully grok what I read, I took a break of a few months and forgot most.
1. Started back up and this time I read the official Rust lang book:  https://doc.rust-lang.org/stable/book/
    1. I bought the printed copy from Nostarch press: [The Rust Programming Language](https://nostarch.com/Rust2018)
        1. This helped me, glad I read it
1. Did several small exercises from https://exercism.org/tracks/rust
1. Began doing the CLI projects from: [Command-Line Rust](https://www.oreilly.com/library/view/command-line-rust/9781098109424/) by Ken Youens-Clark
   1. I *loved* this book, it's been exactly what I needed at this beginning stage, I understand basics of Rust but I need practical exercises so I can begin to "think in Rust".
        1. **Highly Recommend this book**

# Misc Notes
* `clap` is used for command line argument parsing
  * https://docs.rs/clap/3.2.20/clap/
  * https://docs.rs/clap/3.2.20/clap/_derive/_tutorial/index.html
  * https://rust-cli.github.io/book/tutorial/cli-args.html#parsing-cli-arguments-with-clap
* 
# Examples
   * `grrs`:  Simple Grep example from: https://rust-cli.github.io/book/tutorial/index.html
   * `echor`: Reimplementation of Unix [echo](https://www.unix.com/man-page/bsd/1/echo/) 
     * Example:  `cd echor && cargo run -- -n tests/expected/hello1.txt`



# VSCode Notes
1. Installed `rust-analyzer`
2. Enabled format on save via settings.json
   
        "[rust]": {
            "editor.defaultFormatter": "rust-lang.rust-analyzer",
            "editor.formatOnSave": true
        },
