use crate::puzzle::*;
use crate::rule::*;
use itertools::iproduct;
use log::{error, info};
use serde::Serialize;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
struct SolutionRow(Vec<Option<Label>>);

/// The actual solution to a puzzle.
#[derive(Debug, Serialize)]
#[serde(transparent)]
pub struct Solution<'p> {
    /// List of entities that have been solved for.
    /// Each element is a mapping from the category name to the name of the
    /// label for that entity, if it was solved. If that category for the entity
    /// was not solved, the value is None.
    pub labels: Vec<HashMap<&'p str, Option<&'p str>>>,

    /// The puzzle that this is the solution for.
    #[serde(skip)]
    puzzle: &'p Puzzle,
}

impl<'p> fmt::Display for Solution<'p> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use prettytable::*;
        let mut table = Table::new();
        let mut row = Row::empty();
        for cat in 0..self.puzzle.num_categories() {
            row.add_cell(Cell::new(self.puzzle.lookup_category(Category(cat))));
        }
        table.add_row(row);
        for soln_row in &self.labels {
            let mut table_row = Row::empty();
            for cat in 0..self.puzzle.num_categories() {
                let second =
                    soln_row[self.puzzle.lookup_category(Category(cat))];
                let name = second.unwrap_or("");
                table_row.add_cell(Cell::new(name));
            }
            table.add_row(table_row);
        }
        write!(f, "{}", table)
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Cell {
    Empty,
    Yes,
    No,
}

/// The actual Grid that houses the label/label cell interactions.
pub struct Grid<'p> {
    /// A 2-d array with `labels_per_category * num_categories` cells.
    /// The intersection of two labels of the same categories are all
    /// pre-filled with Yes or No the obvious way.
    /// `cells[i][j]` is only valid to be accessed if `i.category <= j.category`,
    /// but `Grid` provides helper functions `at` and `set` so that invariant
    /// is invisible to the user.
    /// TODO: Stop making this take up twice as much memory as it rightfully needs.
    cells: Vec<Vec<Cell>>,

    /// The associated puzzle for the grid.
    puzzle: &'p Puzzle,

    /// The number of labels per category in the puzzle.
    pub labels_per_category: usize,
}

impl<'p> Grid<'p> {
    pub fn new(puzzle: &'p Puzzle) -> Grid {
        let mut cells = vec![];
        let labels_per_category = puzzle.labels_per_category();
        for cat1 in 0..puzzle.num_categories() {
            for l1 in 0..labels_per_category {
                let mut row = vec![];
                for cat2 in 0..puzzle.num_categories() {
                    for l2 in 0..labels_per_category {
                        if cat1 == cat2 {
                            row.push(if l1 == l2 {
                                Cell::Yes
                            } else {
                                Cell::No
                            });
                        } else {
                            row.push(Cell::Empty);
                        }
                    }
                }
                cells.push(row);
            }
        }
        debug_assert_eq!(
            cells[0].len(),
            puzzle.num_categories() * labels_per_category
        );
        debug_assert_eq!(
            cells.len(),
            puzzle.num_categories() * labels_per_category
        );
        Grid {
            cells,
            puzzle,
            labels_per_category,
        }
    }

    /// Return the `(row, col)` indices in `cells` at which to access
    /// `label1` and `label2`'s intersection.
    fn indices(&self, mut label1: Label, mut label2: Label) -> (usize, usize) {
        debug_assert!(label1.category.0 < self.puzzle.num_categories());
        debug_assert!(label2.category.0 < self.puzzle.num_categories());

        if label1.category.0 > label2.category.0 {
            std::mem::swap(&mut label1, &mut label2);
        }

        (
            self.labels_per_category * label1.category.0 + label1.label,
            self.labels_per_category * label2.category.0 + label2.label,
        )
    }

    pub fn at(&self, label1: Label, label2: Label) -> &Cell {
        let (row, col) = self.indices(label1, label2);
        &self.cells[row][col]
    }

    fn at_mut(&mut self, label1: Label, label2: Label) -> &mut Cell {
        let (row, col) = self.indices(label1, label2);
        &mut self.cells[row][col]
    }

    /// Set the cell `(label1, label2)` in the grid to `val`.
    /// Returns `None` if the attempt to set was contradictory,
    /// otherwise returns `Some(changed)` where `changed` is true iff
    /// the cell was changed from its initial value.
    #[must_use]
    pub fn set(
        &mut self,
        label1: Label,
        label2: Label,
        val: Cell,
    ) -> Option<bool> {
        debug_assert_ne!(val, Cell::Empty);
        let c = self.at(label1, label2);
        match *c {
            Cell::Empty => {
                info!(
                    "  {} | {} => {:?}\n",
                    self.puzzle.lookup_label(label1),
                    self.puzzle.lookup_label(label2),
                    val
                );
                *self.at_mut(label1, label2) = val;
                Some(true)
            }
            _ => {
                if val == *c {
                    Some(false)
                } else {
                    error!(
                        "CONTRADICTION: {} | {} => {:?}",
                        self.puzzle.lookup_label(label1),
                        self.puzzle.lookup_label(label2),
                        val
                    );
                    None
                }
            }
        }
    }

    /// Iterate over every category in the grid.
    pub fn categories(&self) -> impl Iterator<Item = Category> {
        let num_categories = self.puzzle.num_categories();
        (0..num_categories).map(Category)
    }

    /// Iterate through every label in the grid.
    pub fn labels(&self) -> impl Iterator<Item = Label> {
        let num_categories = self.puzzle.num_categories();
        let labels_per_category = self.labels_per_category;
        let num_labels = num_categories * self.labels_per_category;
        (0..num_labels).map(move |i| {
            let (c, l) = (i / labels_per_category, i % labels_per_category);
            Label::new(Category(c), l)
        })
    }

    /// Return the user-facing string associated with `label`.
    #[allow(dead_code)]
    pub fn label_str(&self, label: Label) -> &str {
        self.puzzle.lookup_label(label)
    }

    /// Iterate through every cell in the grid, skipping any redundant labels.
    /// If `(x,y)` is included in the iterator, then `(y,x)` is not.
    /// `(x,y)` is not included if `x.category == y.category`.
    pub fn cells(&self) -> impl Iterator<Item = (Label, Label)> {
        let num_categories = self.puzzle.num_categories();
        let labels_per_category = self.labels_per_category;
        let num_labels = num_categories * self.labels_per_category;
        iproduct!(0..num_labels, 0..num_labels).filter_map(move |(i1, i2)| {
            let (c1, l1) = (i1 / labels_per_category, i1 % labels_per_category);
            let (c2, l2) = (i2 / labels_per_category, i2 % labels_per_category);
            if c1 < c2 {
                Some((
                    Label::new(Category(c1), l1),
                    Label::new(Category(c2), l2),
                ))
            } else {
                None
            }
        })
    }
}

struct Solver<'p> {
    puzzle: &'p Puzzle,
    grid: Grid<'p>,
}

impl<'p> Solver<'p> {
    pub fn new(puzzle: &Puzzle) -> Solver {
        let grid = Grid::new(puzzle);
        Solver { puzzle, grid }
    }

    /// Attempt to solve the given puzzle and return the `Solution` for it.
    pub fn solve(mut self) -> Option<Solution<'p>> {
        let mut changed = true;

        let rules: &[Box<dyn Rule>] = &[
            Box::new(ElimOthers {}),
            Box::new(OnlyEmpty {}),
            Box::new(Transitivity {}),
            Box::new(NoByProxy {}),
        ];

        while changed {
            changed = false;
            info!("Running constraints...\n");
            for constraint in self.puzzle.constraints() {
                changed |= constraint.apply(&mut self.grid)?;
            }

            for rule in rules {
                changed |= rule.apply(&mut self.grid)?;
            }
        }
        Some(self.solution())
    }

    /// Create a `Solution` from the current puzzle grid.
    fn solution(&self) -> Solution<'p> {
        let mut map = vec![];
        for l in 0..self.puzzle.labels_per_category() {
            let primary = Label::new(Category(0), l);
            let mut knowns = HashMap::new();
            knowns.insert(
                self.puzzle.lookup_category(Category(0)),
                Some(self.puzzle.lookup_label(primary)),
            );
            for cat in 1..self.puzzle.num_categories() {
                let cat = Category(cat);
                let mut found = false;
                for l in 0..self.puzzle.labels_per_category() {
                    let secondary = Label::new(cat, l);
                    if *self.grid.at(primary, secondary) == Cell::Yes {
                        knowns.insert(
                            self.puzzle.lookup_category(cat),
                            Some(self.puzzle.lookup_label(secondary)),
                        );
                        found = true;
                        break;
                    }
                }
                if !found {
                    knowns.insert(self.puzzle.lookup_category(cat), None);
                }
            }
            map.push(knowns);
        }
        Solution {
            labels: map,
            puzzle: self.puzzle,
        }
    }
}

pub fn solve(puzzle: &Puzzle) -> Option<Solution> {
    let solver = Solver::new(puzzle);
    solver.solve()
}
