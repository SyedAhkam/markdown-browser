use getopts::Options;

use std::env;
use std::path::PathBuf;

use md_browser_server as server;
use md_browser_browser as browser;

fn spawn_server(dir: PathBuf) {
    server::run(dir);
}

fn start_browser() {
    browser::start(); 
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let mut opts = Options::new();
    opts.optopt("s", "server", "spawn a server", "DIR");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => panic!("{}", e.to_string())
    };

    if matches.opt_present("s") {
        let input = matches.opt_str("s").unwrap();
        let path = PathBuf::from(&input);

        path.canonicalize().expect("failed to parse path");

        spawn_server(path);
    }

    start_browser();
}
