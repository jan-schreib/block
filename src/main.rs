#[macro_use]
extern crate structopt;
extern crate capstone;

use std::path::PathBuf;
use structopt::StructOpt;

mod file;
mod asm;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    #[structopt(short = "f", long = "file", parse(from_os_str))]
    file: PathBuf,
}

fn main() {
    let opt = Opt::from_args();
    //if file is executable -> disassemble and diff to old file
    //if not just to regular checksums
    if file::is_executable(&opt.file) {
        println!("true");
    } else {
        println!("false");
    }
}
