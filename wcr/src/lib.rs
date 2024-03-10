use clap::{App, Arg};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
  files: Vec<String>,
  lines: bool,
  words: bool,
  bytes: bool,
  chars: bool,
}

pub fn get_args() -> MyResult<Config> {
  let matches = App::new("wcr")
    .version("0.1.0")
    .author("mnmandahalf")
    .about("Rust wc")
    .arg(
      Arg::with_name("FILE")
        .help("Input file(s)")
        .multiple(true)
        .default_value("-"),
    )
    .arg(
      Arg::with_name("bytes")
        .short("c")
        .long("bytes")
        .help("Show byte count")
    )
    .arg(
      Arg::with_name("chars")
        .short("m")
        .long("chars")
        .help("Show character count")
        .conflicts_with("bytes")
    )
    .arg(
      Arg::with_name("lines")
        .short("l")
        .long("lines")
        .help("Show line count"),
    )
    .arg(
      Arg::with_name("version")
        .short("V")
        .long("version")
        .help("Prints version information"),
    )
    .arg(
      Arg::with_name("words")
        .short("w")
        .long("words")
        .help("Show word count"),
    )
    .get_matches();
  Ok(Config {
    files: matches.values_of_lossy("files").unwrap_or_default(),
    lines: matches.is_present("lines"),
    words: matches.is_present("words"),
    bytes: matches.is_present("bytes"),
    chars: matches.is_present("chars"),
  })
}

pub fn run(config: Config) -> MyResult<()> {
  println!("{:#?}", config);
  Ok(())
}