#![allow(unused)]

use std::{
    path::PathBuf,
    io::{ self, Write, BufReader, BufRead },
    fs::File,
    error::Error,
    thread,
    time::Duration
};
use clap::Parser;
use anyhow::{ Context, Result };

#[derive(Debug, Parser)]
struct Cli {
    pattern: String,
    path: PathBuf,
}

fn main() -> Result<()> {
    let pb = indicatif::ProgressBar::new(100);

    for i in 0..100 {
        thread::sleep(Duration::from_millis(1000));
        pb.println(format!("[+] finished #{}", i));
        pb.inc(1);
    }

    pb.finish_with_message("done");

    // let stdout = io::stdout();
    // let mut handle = stdout.lock();

    // let path = "test.txt";
    // let args = Cli::parse();
    // let file = File::open(args.path)
    //     .with_context(|| format!("could not read file {}", path))?;
    // let reader: BufReader<File> = BufReader::new(file);

    // for result in reader.lines() {
    //     let line = match result {
    //         Ok(line) => { line },
    //         Err(error) => { return Err(error.into()); }
    //     };

    //     if line.contains(&args.pattern) {
    //         writeln!(handle, "{}", line);
    //     }
    // }

    Ok(())
}
