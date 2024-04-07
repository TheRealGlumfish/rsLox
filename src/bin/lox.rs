use std::cmp::Ordering;
use std::env;
use std::process;

fn main() {
    let mut args = env::args();
    let err = match args.len().cmp(&2) {
        Ordering::Less => rslox::run_prompt(),
        Ordering::Equal => rslox::run_file(args.nth(1).unwrap().as_str()),
        Ordering::Greater => {
            println!("Usage: rslox [script]");
            process::exit(64);
        }
    };
    if let Err(err) = err {
        println!("Internal error: {err}");
        process::exit(74);
    }
}
