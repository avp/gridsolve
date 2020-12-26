extern crate gridsolve;

use gridsolve::{solve, Puzzle};
use log::{error, LevelFilter};
use log4rs::{
    append::console::ConsoleAppender,
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "gridsolve", about = "A grid logic puzzle solver")]
struct Opt {
    /// Show step-by-step solving method
    #[structopt(short, long)]
    verbose: bool,

    /// Output solution as JSON
    #[structopt(long)]
    json: bool,

    /// Input file, formatted as a grid puzzle
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

fn main() {
    let opt = Opt::from_args();

    let stdout: ConsoleAppender = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{m}")))
        .build();
    let log_config = log4rs::config::Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(if opt.verbose {
            LevelFilter::Info
        } else {
            LevelFilter::Error
        }))
        .unwrap();
    log4rs::init_config(log_config).unwrap();

    let puzzle = match Puzzle::from_file(&opt.input) {
        Ok(puzzle) => puzzle,
        Err(err) => {
            error!("{}\n", err);
            return;
        }
    };
    let solution = match solve(&puzzle) {
        Some(solution) => solution,
        None => {
            error!("Clues are contradictory\n");
            return;
        }
    };

    if opt.json {
        println!("{}", serde_json::to_string(&solution).unwrap());
    } else {
        println!("{}", solution);
    }
}
