#![allow(clippy::many_single_char_names)]

use crate::puzzle::*;
use crate::rule::Rule;
use crate::solver::{Cell, Grid};
use log::info;

#[derive(Debug)]
pub enum ConstraintKind {
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

#[derive(Debug)]
pub struct Constraint {
    pub kind: ConstraintKind,

    pub name: String,
}

impl Constraint {
    #[must_use]
    fn apply_after_at_least<'p>(
        &self,
        grid: &mut Grid<'p>,
        puzzle: &Puzzle,
        x: Label,
        c: Category,
        y: Label,
        n: usize,
    ) -> Option<bool> {
        let mut changed = false;
        // No overlap if x is after y.
        changed |= grid.set_with_callback(x, y, Cell::No, || {
            info!(
                "    Constraint {} => {} ({}) must appear after {} ({})\n",
                self.name,
                puzzle.lookup_label(x),
                puzzle.lookup_category(x.category),
                puzzle.lookup_label(y),
                puzzle.lookup_category(y.category),
            );
        })?;

        // x must appear `n` after the appearance of y.
        for i in 0..grid.labels_per_category {
            if i < n {
                changed |= grid.set_with_callback(x, Label::new(c, i), Cell::No, || {
                    info!(
                        "    Constraint {} => There must be {} in ({}) before {} ({})\n",
                        self.name,
                        n,
                        puzzle.lookup_category(c),
                        puzzle.lookup_label(x),
                        puzzle.lookup_category(x.category),
                    );
                })?;
            } else {
                let l = Label::new(c, i - n);
                if *grid.at(y, l) == Cell::No {
                    grid.set_with_callback(x, Label::new(c, i), Cell::No, || {
                        info!(
                            "    Constraint {} => {} ({}) conflicts with {} ({}) due to distance\n",
                            self.name,
                            puzzle.lookup_label(x),
                            puzzle.lookup_category(x.category),
                            puzzle.lookup_label(y),
                            puzzle.lookup_category(y.category),
                        );
                    })?;
                } else {
                    break;
                }
            }
        }

        // y must appear before the appearance of x.
        for i in (0..grid.labels_per_category).rev() {
            if i + n >= grid.labels_per_category {
                changed |= grid.set_with_callback(y, Label::new(c, i), Cell::No, || {
                    info!(
                        "    Constraint {} => There must be {} in ({}) after {} ({})\n",
                        self.name,
                        n,
                        puzzle.lookup_category(c),
                        puzzle.lookup_label(y),
                        puzzle.lookup_category(y.category),
                    );
                })?;
            } else {
                let l = Label::new(c, i + n);
                if *grid.at(x, l) == Cell::No {
                    grid.set_with_callback(y, Label::new(c, i), Cell::No, || {
                        info!(
                            "    Constraint {} => {} ({}) conflicts with {} ({}) due to distance\n",
                            self.name,
                            puzzle.lookup_label(y),
                            puzzle.lookup_category(y.category),
                            puzzle.lookup_label(x),
                            puzzle.lookup_category(x.category),
                        );
                    })?;
                } else {
                    break;
                }
            }
        }

        Some(changed)
    }

    #[must_use]
    fn apply_xor<'p>(
        &self,
        grid: &mut Grid<'p>,
        puzzle: &Puzzle,
        x: Label,
        y: Label,
        z: Label,
    ) -> Option<bool> {
        let mut changed = false;
        changed |= grid.set(y, z, Cell::No)?;

        // If one of them is No, the other must be Yes.
        if *grid.at(x, y) == Cell::No {
            changed |= grid.set_with_callback(x, z, Cell::Yes, || {
                info!(
                    "    Constraint {} => {} ({}) not with {} ({}), so must be with {} ({})\n",
                    self.name,
                    puzzle.lookup_label(x),
                    puzzle.lookup_category(x.category),
                    puzzle.lookup_label(y),
                    puzzle.lookup_category(y.category),
                    puzzle.lookup_label(z),
                    puzzle.lookup_category(z.category),
                );
            })?;
        } else if *grid.at(x, z) == Cell::No {
            changed |= grid.set_with_callback(x, y, Cell::Yes, || {
                info!(
                    "    Constraint {} => {} ({}) not with {} ({}), so must be with {} ({})\n",
                    self.name,
                    puzzle.lookup_label(x),
                    puzzle.lookup_category(x.category),
                    puzzle.lookup_label(z),
                    puzzle.lookup_category(z.category),
                    puzzle.lookup_label(y),
                    puzzle.lookup_category(y.category),
                );
            })?;
        }

        // If one of them is Yes, the other must be No.
        if *grid.at(x, y) == Cell::Yes {
            changed |= grid.set_with_callback(x, z, Cell::No, || {
                info!(
                    "    Constraint {} => {} ({}) with {} ({}), so can't be with {} ({})\n",
                    self.name,
                    puzzle.lookup_label(x),
                    puzzle.lookup_category(x.category),
                    puzzle.lookup_label(y),
                    puzzle.lookup_category(y.category),
                    puzzle.lookup_label(z),
                    puzzle.lookup_category(z.category),
                );
            })?;
        } else if *grid.at(x, z) == Cell::Yes {
            changed |= grid.set_with_callback(x, y, Cell::No, || {
                info!(
                    "    Constraint {} => {} ({}) with {} ({}), so can't be with {} ({})\n",
                    self.name,
                    puzzle.lookup_label(x),
                    puzzle.lookup_category(x.category),
                    puzzle.lookup_label(z),
                    puzzle.lookup_category(z.category),
                    puzzle.lookup_label(y),
                    puzzle.lookup_category(y.category),
                );
            })?;
        }

        // Now search for any existing labels which are neither,
        // and eliminate them as possibilities.
        for w in grid.labels() {
            if w.category == x.category {
                continue;
            }
            if *grid.at(y, w) == Cell::No && *grid.at(z, w) == Cell::No {
                changed |= grid.set_with_callback(x, w, Cell::No, || {
                    info!(
                        "    Constraint {} => {} ({}) must be either {} ({}) or {} ({}), \
conflicts with {} ({}) which is with neither\n",
                        self.name,
                        puzzle.lookup_label(x),
                        puzzle.lookup_category(x.category),
                        puzzle.lookup_label(z),
                        puzzle.lookup_category(z.category),
                        puzzle.lookup_label(y),
                        puzzle.lookup_category(y.category),
                        puzzle.lookup_label(w),
                        puzzle.lookup_category(y.category),
                    );
                })?;
            }
            if *grid.at(y, w) == Cell::Yes && *grid.at(z, w) == Cell::Yes {
                changed |= grid.set_with_callback(x, w, Cell::No, || {
                    info!(
                        "    Constraint {} => {} ({}) must be one of {} ({}) or {} ({}), \
conflicts with {} ({}) which is with both\n",
                        self.name,
                        puzzle.lookup_label(x),
                        puzzle.lookup_category(x.category),
                        puzzle.lookup_label(z),
                        puzzle.lookup_category(z.category),
                        puzzle.lookup_label(y),
                        puzzle.lookup_category(y.category),
                        puzzle.lookup_label(w),
                        puzzle.lookup_category(y.category),
                    );
                })?;
            }
        }
        Some(changed)
    }
}

impl Rule for Constraint {
    fn apply<'p>(&self, grid: &mut Grid<'p>, puzzle: &'p Puzzle) -> Option<bool> {
        let mut changed = false;
        match &self.kind {
            &ConstraintKind::Yes(x, y) => {
                changed |= grid.set_with_callback(x, y, Cell::Yes, || {
                    info!(
                        "Constraint {} => Direct confirmation on {} ({}) and {} ({})",
                        self.name,
                        puzzle.lookup_label(x),
                        puzzle.lookup_category(x.category),
                        puzzle.lookup_label(y),
                        puzzle.lookup_category(y.category),
                    );
                })?;
            }

            &ConstraintKind::No(x, y) => {
                changed |= grid.set_with_callback(x, y, Cell::No, || {
                    info!(
                        "Constraint {} => Direct elimination on {} ({}) and {} ({})",
                        self.name,
                        puzzle.lookup_label(x),
                        puzzle.lookup_category(x.category),
                        puzzle.lookup_label(y),
                        puzzle.lookup_category(y.category),
                    );
                })?;
            }

            &ConstraintKind::After(x, c, y) => {
                changed |= self.apply_after_at_least(grid, puzzle, x, c, y, 1)?;
            }

            &ConstraintKind::AfterAtLeast(x, c, y, n) => {
                changed |= self.apply_after_at_least(grid, puzzle, x, c, y, n)?;
            }

            &ConstraintKind::AfterExactly(x, c, y, n) => {
                changed |= grid.set(x, y, Cell::No)?;
                for i in 0..grid.labels_per_category {
                    if i < n {
                        changed |= grid.set_with_callback(x, Label::new(c, i), Cell::No, || {
                            info!(
                                "    Constraint {} => {} ({}) must have {} before in ({})\n",
                                self.name,
                                puzzle.lookup_label(x),
                                puzzle.lookup_category(x.category),
                                n,
                                puzzle.lookup_category(c),
                            );
                        })?;
                        continue;
                    }
                    let l = Label::new(c, i - n);
                    if *grid.at(y, l) == Cell::No {
                        changed |= grid.set_with_callback(x, Label::new(c, i), Cell::No, || {
                            info!(
                                "    Constraint {} => {} ({}) must have {} after in ({})\n",
                                self.name,
                                puzzle.lookup_label(x),
                                puzzle.lookup_category(x.category),
                                n,
                                puzzle.lookup_category(c),
                            );
                        })?;
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

            &ConstraintKind::Distance(x, c, y, n) => {
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
                        // Can't go `n` higher, so just check lower.
                        let lo_x = *grid.at(x, Label::new(c, i - n));
                        let lo_y = *grid.at(y, Label::new(c, i - n));
                        if lo_x != Cell::Empty {
                            changed |= grid.set(y, cur, lo_x)?;
                        } else if lo_y != Cell::Empty {
                            changed |= grid.set(x, cur, lo_y)?;
                        }
                    } else {
                        // TODO: Check both directions.
                    }
                }
            }

            &ConstraintKind::Or(x, y, z) => {
                // If one of them is No, the other must be Yes.
                if *grid.at(x, y) == Cell::No {
                    changed |= grid.set(x, z, Cell::Yes)?;
                } else if *grid.at(x, z) == Cell::No {
                    changed |= grid.set(x, y, Cell::Yes)?;
                }

                // Now search for any existing labels which are neither,
                // and eliminate them as possibilities.
                for attempt in grid.labels() {
                    let first = (attempt.category == y.category && attempt.label != y.label)
                        || (attempt.category != y.category && *grid.at(attempt, y) == Cell::No);
                    let second = (attempt.category == z.category && attempt.label != z.label)
                        || (attempt.category != z.category && *grid.at(attempt, z) == Cell::No);
                    if first && second {
                        changed |= grid.set(attempt, x, Cell::No)?;
                    }
                }
            }

            &ConstraintKind::Xor(x, y, z) => {
                changed |= self.apply_xor(grid, puzzle, x, y, z)?;
            }

            &ConstraintKind::TwoByTwo(x1, x2, y1, y2) => {
                // TwoByTwo is equivalent to having two Xors and a No.
                changed |= grid.set(x1, x2, Cell::No)?;
                changed |= grid.set(y1, y2, Cell::No)?;
                changed |= self.apply_xor(grid, puzzle, x1, y1, y2)?;
                changed |= self.apply_xor(grid, puzzle, x2, y1, y2)?;
            }

            ConstraintKind::ExactlyOne(constraints) => {
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
