extern crate gridsolve;

use gridsolve::{solve, Puzzle};
use std::collections::HashSet;
use std::path::PathBuf;

macro_rules! check_solution {
    ($puzzle: expr, $solution: expr, $prim: expr, $($sec: expr), +) => {
        let mut found = false;
        for soln_row in &$solution.labels {
            let all = soln_row.values().collect::<HashSet<&Option<&str>>>();
            let expected = [Some($prim),
                $(
                    Some($sec),
                )+].iter().collect::<HashSet<&Option<&str>>>();
            if all.contains(&Some($prim)) {
                assert_eq!(&all, &expected);
                found = true;
            }
        }
        assert!(found, "Invalid row: {}", $prim);
    };
}

#[test]
fn test_simple() {
    let puz = Puzzle::from_file(&PathBuf::from("puzzles/simple.txt")).unwrap();
    let sol = solve(&puz).unwrap();
    check_solution!(puz, sol, "Angela", "Germany", "1954");
    check_solution!(puz, sol, "Donald", "United States", "1946");
    check_solution!(puz, sol, "Leo", "Ireland", "1979");
}
