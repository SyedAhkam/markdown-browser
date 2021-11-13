use std::env;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("An argument was expected.");

        std::process::exit(-1);
    }

    let input = args[1].to_owned();
    let path = Path::new(&input);

    path.canonicalize().expect("failed to parse path");

    server::run(path);
}
