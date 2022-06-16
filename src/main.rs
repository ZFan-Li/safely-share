mod datamanip;
mod io;
mod opt;
mod tests;

use opt::Opt;

use structopt::StructOpt;

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
}
