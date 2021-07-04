#![allow(clippy::many_single_char_names)]

use crate::puzzle::*;
use crate::rule::Rule;
use crate::solver::{Cell, Grid};

#[derive(Debug)]
pub enum Constraint {
    /// Yes(x, y) ==> (x, y) == No
    Yes(Label, Label),

    /// No(x, y) ==> (x, y) == Yes
    No(Label, Label),

    /// Or(x, y, z) ==> (x, y) == Yes || (x, z) == Yes
    Or(Label, Label, Label),

    /// Xor(x, y, z) ==> (x, y) == Yes ^ (x, z) == Yes
    Xor(Label, Label, Label),

    /// After(x, C, y) ==> x is after y in category C
    After(Label, Category, Label),

    /// AfterExactly(x, C, y, n) ==> x is at least n spots after y in category C
    AfterAtLeast(Label, Category, Label, usize),

    /// AfterExactly(x, C, y, n) ==> x is exactly n spots after y in category C
    AfterExactly(Label, Category, Label, usize),

    /// Distance(x, C, y, n) ==> x is n spots before or after y in category C
    Distance(Label, Category, Label, usize),

    /// TwoByTwo(x1, x2, y1, y2) ==> either (x1,y1) and (x2,y2)
    ///                                 XOR (x1,y2) and (x2,y1)
    TwoByTwo(Label, Label, Label, Label),

    /// ExactlyOne((x1,y1), ..., (xn, yn)) ==> exists exactly one i such that
    ///                                        (xi, yi)
    ExactlyOne(Vec<(Label, Label)>),
}

impl Rule for Constraint {
    fn apply(&self, grid: &mut Grid) -> Option<bool> {
        let mut changed = false;
        match self {
            &Constraint::Yes(x, y) => {
                changed |= grid.set(x, y, Cell::Yes)?;
            }

            &Constraint::No(x, y) => {
                changed |= grid.set(x, y, Cell::No)?;
            }

            &Constraint::After(x, c, y) => {
                changed |= Constraint::AfterAtLeast(x, c, y, 1).apply(grid)?;
            }

            &Constraint::AfterAtLeast(x, c, y, n) => {
                // No overlap if x is after y.
                changed |= grid.set(x, y, Cell::No)?;

                // x must appear `n` after the appearance of y.
                for i in 0..grid.labels_per_category {
                    if i < n {
                        changed |= grid.set(x, Label::new(c, i), Cell::No)?;
                    } else {
                        let l = Label::new(c, i - n);
                        if *grid.at(y, l) == Cell::No {
                            grid.set(x, Label::new(c, i), Cell::No)?;
                        } else {
                            break;
                        }
                    }
                }

                // y must appear before the appearance of x.
                for i in (0..grid.labels_per_category).rev() {
                    if i + n >= grid.labels_per_category {
                        changed |= grid.set(y, Label::new(c, i), Cell::No)?;
                    } else {
                        let l = Label::new(c, i + n);
                        if *grid.at(x, l) == Cell::No {
                            grid.set(y, Label::new(c, i), Cell::No)?;
                        } else {
                            break;
                        }
                    }
                }
            }

            &Constraint::AfterExactly(x, c, y, n) => {
                changed |= grid.set(x, y, Cell::No)?;
                for i in 0..grid.labels_per_category {
                    if i < n {
                        changed |= grid.set(x, Label::new(c, i), Cell::No)?;
                        continue;
                    }
                    let l = Label::new(c, i - n);
                    if *grid.at(y, l) == Cell::No {
                        changed |= grid.set(x, Label::new(c, i), Cell::No)?;
                    }
                }
                for i in (0..grid.labels_per_category).rev() {
                    if i + n > grid.labels_per_category - 1 {
                        changed |= grid.set(y, Label::new(c, i), Cell::No)?;
                        continue;
                    }
                    let l = Label::new(c, i + n);
                    if *grid.at(x, l) == Cell::No {
                        changed |= grid.set(y, Label::new(c, i), Cell::No)?;
                    }
                }
            }

            &Constraint::Distance(x, c, y, n) => {
                changed |= grid.set(x, y, Cell::No)?;
                for i in 0..grid.labels_per_category {
                    let cur = Label::new(c, i);
                    if i < n {
                        // Can't go `n` lower, so just check higher.
                        let hi_x = *grid.at(x, Label::new(c, i + n));
                        let hi_y = *grid.at(y, Label::new(c, i + n));
                        // If we can only go in one direction, then if the other
                        // cell is filled, that determines the current cell.
                        // If `i == 0` and `n == 1`, then if (x, 1) is No,
                        // (y, 0) must also be No. If (x, 1) is Yes, then (y, 0)
                        // must be Yes.
                        if hi_x != Cell::Empty {
                            changed |= grid.set(y, cur, hi_x)?;
                        } else if hi_y != Cell::Empty {
                            changed |= grid.set(x, cur, hi_y)?;
                        }
                    } else if i + n >= grid.labels_per_category {
                        // TODO: Can't go `n` higher, so just check lower.
                    } else {
                        // TODO: Check both directions.
                    }
                }
            }

            &Constraint::Or(x, y, z) => {
                // If one of them is No, the other must be Yes.
                if *grid.at(x, y) == Cell::No {
                    changed |= grid.set(x, z, Cell::Yes)?;
                } else if *grid.at(x, z) == Cell::No {
                    changed |= grid.set(x, y, Cell::Yes)?;
                }

                // Now search for any existing labels which are neither,
                // and eliminate them as possibilities.
                for attempt in grid.labels() {
                    let first = (attempt.category == y.category
                        && attempt.label != y.label)
                        || (attempt.category != y.category
                            && *grid.at(attempt, y) == Cell::No);
                    let second = (attempt.category == z.category
                        && attempt.label != z.label)
                        || (attempt.category != z.category
                            && *grid.at(attempt, z) == Cell::No);
                    if first && second {
                        changed |= grid.set(attempt, x, Cell::No)?;
                    }
                }
            }

            &Constraint::Xor(x, y, z) => {
                changed |= grid.set(y, z, Cell::No)?;

                // If one of them is No, the other must be Yes.
                if *grid.at(x, y) == Cell::No {
                    changed |= grid.set(x, z, Cell::Yes)?;
                } else if *grid.at(x, z) == Cell::No {
                    changed |= grid.set(x, y, Cell::Yes)?;
                }

                // If one of them is Yes, the other must be No.
                if *grid.at(x, y) == Cell::Yes {
                    changed |= grid.set(x, z, Cell::No)?;
                } else if *grid.at(x, z) == Cell::Yes {
                    changed |= grid.set(x, y, Cell::No)?;
                }

                // Now search for any existing labels which are neither,
                // and eliminate them as possibilities.
                for w in grid.labels() {
                    if w.category == x.category {
                        continue;
                    }
                    if *grid.at(y, w) == Cell::No && *grid.at(z, w) == Cell::No
                    {
                        changed |= grid.set(x, w, Cell::No)?;
                    }
                    if *grid.at(y, w) == Cell::Yes
                        && *grid.at(z, w) == Cell::Yes
                    {
                        changed |= grid.set(x, w, Cell::No)?;
                    }
                }
            }

            &Constraint::TwoByTwo(x1, x2, y1, y2) => {
                // TwoByTwo is equivalent to having two Xors and a No.
                changed |= grid.set(x1, x2, Cell::No)?;
                changed |= grid.set(y1, y2, Cell::No)?;
                changed |= Constraint::Xor(x1, y1, y2).apply(grid)?;
                changed |= Constraint::Xor(x2, y1, y2).apply(grid)?;
            }

            Constraint::ExactlyOne(constraints) => {
                let mut found_yes = false;
                let mut num_no = 0;

                for &(xi, yi) in constraints {
                    match *grid.at(xi, yi) {
                        Cell::Yes => {
                            found_yes = true;
                            break;
                        }
                        Cell::No => {
                            num_no += 1;
                        }
                        Cell::Empty => {}
                    }
                }

                if found_yes {
                    // If we found a Yes anywhere, then all other cells must be No.
                    for &(xi, yi) in constraints {
                        match *grid.at(xi, yi) {
                            Cell::Yes => {}
                            _ => {
                                changed |= grid.set(xi, yi, Cell::No)?;
                            }
                        }
                    }
                } else if num_no == constraints.len() - 1 {
                    // If all but one cell is No, then the remaining cell must be Yes.
                    for &(xi, yi) in constraints {
                        if *grid.at(xi, yi) == Cell::Empty {
                            changed |= grid.set(xi, yi, Cell::Yes)?;
                        }
                    }
                }
            }
        }
        Some(changed)
    }
}
