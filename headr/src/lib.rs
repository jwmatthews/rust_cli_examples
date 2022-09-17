use clap::{ArgGroup, Parser};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)] // Read from `Cargo.toml`
#[clap(group(ArgGroup::new("length")
    .required(false)
    .args(&["lines", "bytes"]),
))]

pub struct Config {
    #[clap(default_values = &["-"])]
    files: Vec<String>,

    #[clap(short = 'n', long = "lines", default_value = "10", value_parser = ensure_pos_int, action)]
    lines: usize,

    #[clap(short = 'c', long = "bytes", value_parser = ensure_pos_int, action)]
    bytes: Option<usize>,
}

fn ensure_pos_int(s: &str) -> Result<usize, String> {
    let num: usize = s
        .parse()
        .map_err(|_| format!("{} isn't a valid positive number", s))?;
    match num {
        n if n >= 1 => Ok(n),
        _ => Err(format!("{} needs to be positive, not zero", num)),
    }
}

#[test]
fn test_ensure_pos_int() {
    // 3 is an OK integer
    let res = ensure_pos_int("3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3);

    // Any string is an error
    let res = ensure_pos_int("foo");
    assert!(res.is_err());
    assert_eq!(
        res.unwrap_err().to_string(),
        "foo isn't a valid positive number".to_string()
    );

    // A zero is an error
    let res = ensure_pos_int("0");
    assert!(res.is_err());
    assert_eq!(
        res.unwrap_err().to_string(),
        "0 needs to be positive, not zero".to_string()
    );
}

pub fn run(config: Config) -> MyResult<()> {
    //dbg!(&config);
    let num_files = config.files.len();
    for (file_num, filename) in config.files.iter().enumerate() {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(mut file) => {
                if num_files > 1 {
                    println!(
                        "{}==> {} <==",
                        if file_num > 0 { "\n" } else { "" },
                        filename
                    );
                }
                if let Some(num_bytes) = config.bytes {
                    let mut handle = file.take(num_bytes as u64);
                    let mut buffer = vec![0; num_bytes];
                    let bytes_read = handle.read(&mut buffer)?;
                    print!("{}", String::from_utf8_lossy(&buffer[..bytes_read]));
                } else {
                    let mut line = String::new();
                    for _ in 0..config.lines {
                        let bytes = file.read_line(&mut line)?;
                        if bytes == 0 {
                            break;
                        }
                        print!("{}", line);
                        line.clear();
                    }
                }
            }
        }
    }
    Ok(())
}

pub fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
