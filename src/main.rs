mod datamanip;
mod io;
mod opt;
mod tests;

use std::{
    fs::{File, OpenOptions},
    io::{stdin, stdout, BufReader, BufWriter},
};

use io::{gather, share};
use opt::{expand_path, get_length, Opt};

use structopt::StructOpt;

fn main() -> std::io::Result<()> {
    match opt::Opt::from_args() {
        Opt::Share {
            input,
            with,
            pieces,
        } => {
            let default_length = with.len();
            let paths = expand_path(with, pieces.unwrap_or(default_length))?;
            let mut writers: Vec<_> = Vec::with_capacity(paths.len());
            for path in paths {
                let file = OpenOptions::new()
                    .create(true)
                    .write(true)
                    .truncate(true)
                    .open(path)?;
                let w = BufWriter::new(file);
                writers.push(w);
            }
            if let Some(path) = input {
                let file = File::open(path)?;
                let mut reader = BufReader::new(file);
                share(&mut reader, &mut writers)
            } else {
                let mut reader = BufReader::new(stdin());
                share(&mut reader, &mut writers)
            }
        }
        Opt::Gather { input, into } => {
            get_length(&input)?;
            let mut readers = Vec::with_capacity(input.len());
            for path in input {
                let file = File::open(path)?;
                let r = BufReader::new(file);
                readers.push(r);
            }
            if let Some(path) = into {
                let file = OpenOptions::new()
                    .create(true)
                    .write(true)
                    .truncate(true)
                    .open(path)?;
                let mut writer = BufWriter::new(file);
                gather(&mut readers, &mut writer)
            } else {
                let mut writer = BufWriter::new(stdout());
                gather(&mut readers, &mut writer)
            }
        }
    }
}
