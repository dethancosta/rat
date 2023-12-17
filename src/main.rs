pub mod rat;

use std::{env, io};
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("No input provided");
        std::process::exit(1);
    } else if args[1] == "-" {
        rat::get_contents(io::stdin()).unwrap()
    } else {
        let mut f: File = File::open(&args[1]).unwrap();
        rat::get_contents(&mut f).unwrap()
    }
}
