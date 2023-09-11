use std::env;
use std::process;

fn main() {
    let mut args = env::args();
    let err = if args.len() > 2 {
        println!("Usage: rslox [script]");
        process::exit(64);
    } else if args.len() == 2 {
        rslox::run_file(args.nth(1).unwrap().as_str())
    } else {
        rslox::run_prompt()
    };
    if let Err(err) = err {
        println!("Internal error: {err}");
        process::exit(74);
    }
}
