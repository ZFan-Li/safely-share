use std::{
    io::{self, ErrorKind},
    path::PathBuf,
};

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "safely", about = "Share data with others safely.")]
pub enum Opt {
    /// Split a plaintext to ciphers.
    ///
    /// Source is provided by argument `input`, which point to a
    /// existed file path. Read source from stdin if this argument is
    /// not provided. One of `with` and `pieces` have to be provided,
    /// which indicates how many output should be generated. However,
    /// if both of them are provided, `pieces` must exceed the length
    /// of list in `with`, and then unspecified file names will be up
    /// counted from 0, otherwise a error will be raised.
    Share {
        /// Plaintext which will be shared.
        #[structopt(parse(from_os_str))]
        input: Option<PathBuf>,
        /// A List of file name which will be filled with enciphered
        /// text.
        #[structopt(long, short)]
        with: Vec<PathBuf>,
        /// Number of enciphered texts in which INPUT will be split.
        #[structopt(long, short)]
        pieces: Option<usize>,
    },
    /// Merge ciphers and decode them to plaintext.
    ///
    /// All files in `input` should have equal length. If they do
    /// have, a result will be produced anyway, but the result will be
    /// right(readable or executable) if and only if sources from
    /// `input` are complete and correct. Print result to stdout if no
    /// `into` is given.
    Gather {
        /// Path to ciphers.
        #[structopt(parse(from_os_str))]
        input: Vec<PathBuf>,
        /// Path to output.
        #[structopt(long, short)]
        into: Option<PathBuf>,
    },
}

pub fn expand_path(mut given_path: Vec<PathBuf>, total_number: usize) -> io::Result<Vec<PathBuf>> {
    match total_number.cmp(&given_path.len()) {
        std::cmp::Ordering::Less => Err(io::Error::new(
            ErrorKind::InvalidInput,
            format!(
                "Arguments aren't compatible. Provide {} path(s), but need {} piece(s) of output.",
                given_path.len(),
                &total_number
            ),
        )),
        std::cmp::Ordering::Equal => Ok(given_path),
        std::cmp::Ordering::Greater => {
            let delta = total_number - given_path.len();
            given_path.extend((0..delta).map(|n| PathBuf::from(n.to_string())));
            Ok(given_path)
        }
    }
}

pub fn get_length(paths: &Vec<PathBuf>) -> io::Result<u64> {
    let mut len: u64 = 0;
    for (idx, path) in paths.iter().enumerate() {
        let md = path.metadata()?;
        if idx == 0 || md.len() == len {
            len = md.len();
        } else {
            let err = io::Error::new(ErrorKind::InvalidData, "Ciphers must have equal length");
            return Err(err);
        }
    }
    Ok(len)
}
