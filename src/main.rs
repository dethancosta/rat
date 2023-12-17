pub mod rat;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    rat::get_contents(&args[1]).unwrap()
}
