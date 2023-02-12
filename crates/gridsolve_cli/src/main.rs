extern crate gridsolve;

use gridsolve::{solve, Puzzle, Solution};
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

fn pretty_solution(solution: &Solution) -> prettytable::Table {
    use prettytable::*;
    let mut table = Table::new();
    let mut row = Row::empty();
    for cat in solution.puzzle.categories() {
        row.add_cell(Cell::new(solution.puzzle.lookup_category(cat)));
    }
    table.add_row(row);
    for soln_row in &solution.labels {
        let mut table_row = Row::empty();
        for cat in solution.puzzle.categories() {
            let second = soln_row[solution.puzzle.lookup_category(cat)];
            let name = second.unwrap_or("");
            table_row.add_cell(Cell::new(name));
        }
        table.add_row(table_row);
    }
    table
}

fn main() {
    let opt = Opt::from_args();

    let puzzle = match Puzzle::from_file(&opt.input) {
        Ok(puzzle) => puzzle,
        Err(err) => {
            eprintln!("{}\n", err);
            return;
        }
    };
    let solution = match solve(&puzzle) {
        Some(solution) => solution,
        None => {
            eprintln!("Clues are contradictory\n");
            return;
        }
    };

    if opt.verbose {
        for step in &solution.steps {
            println!(
                "{} ({}, {}) [{}]",
                if step.yes { "\u{2705}" } else { " \u{274c}" },
                step.label1,
                step.label2,
                &step.description.trim()
            );
        }
    }

    if opt.json {
        println!("{}", serde_json::to_string(&solution).unwrap());
    } else {
        println!("{}", pretty_solution(&solution));
    }
}
