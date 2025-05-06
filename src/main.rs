use minigrep::Config;
use minigrep::run;
use std::{env, process};
fn main() {
    let args: Vec<String> = env::args().collect();
    let config: Config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    run(config).unwrap_or_else(|err| println!("Problem While Opening File:{err}"));
}
