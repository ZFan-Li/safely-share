mod datamanip;
mod io;
mod tests;

use std::path::PathBuf;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "safely", about = "Share data with others safely.")]
enum Opt {
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
    /// Merge ciphers to plaintext.
    ///
    /// All files in `input` should have equal length. If they do
    /// have, a result will be produced anyway, but the result will be
    /// right(usable or readable) if and only if sources from `input`
    /// are complete and correct.
    Gather {
        /// Path to ciphers.
        #[structopt(parse(from_os_str))]
        input: Vec<PathBuf>,
        /// Path to output.
        #[structopt(long, short)]
        output: PathBuf,
    },
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
}
