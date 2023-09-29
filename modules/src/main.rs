use std::error::Error;
use std::process;

use structopt::StructOpt;

mod lib;
use lib::Opt;

fn main() {
    if let Err(e) = try_main() {
        eprintln!("{}", e);
        process::exit(1);
    }
}

fn try_main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();
    lib::run(opt)?;

    Ok(())
}
