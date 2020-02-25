#![forbid(unsafe_code)]

mod args;
mod build_info;

use clap::ArgMatches;
use std::env;
use std::io;
use std::process;

fn main() {
    process::exit(match run(args::parse(env::args())) {
        None => exitcode::OK,
        Some(error) => {
            println!("{}", error);
            error.raw_os_error().unwrap_or(exitcode::SOFTWARE)
        }
    })
}

fn run(args: ArgMatches<'static>) -> Option<io::Error> {
    println!("{:?}", args);
    None
}
