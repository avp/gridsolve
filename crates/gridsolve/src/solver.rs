use crate::puzzle::*;
use crate::rule::*;
use itertools::iproduct;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug)]
struct SolutionRow(Vec<Option<Label>>);

#[derive(Debug, Serialize)]
#[serde(transparent)]
pub struct Step {
    description: String,
}

impl Step {
    pub fn new(description: String) -> Self {
        Step { description }
    }
}

/// The actual solution to a puzzle.
/// Stores a reference to the puzzle itself.
#[derive(Debug, Serialize)]
pub struct Solution<'p> {
    /// List of entities that have been solved for.
    /// Each element is a mapping from the category name to the name of the
    /// label for that entity, if it was solved. If that category for the entity
    /// was not solved, the value is None.
    pub labels: Vec<HashMap<&'p str, Option<&'p str>>>,

    pub steps: Vec<Step>,

    /// The puzzle that this is the solution for.
    #[serde(skip)]
    pub puzzle: &'p Puzzle,
}

/// Cell in the "grid puzzle" format for the logic puzzle.
/// Empty by default.
/// `Yes` and `No` indicate definite confirmations of whether or not the corresponding grid
/// has been filled in.
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
    pub puzzle: &'p Puzzle,

    pub steps: Vec<Step>,

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
                            row.push(if l1 == l2 { Cell::Yes } else { Cell::No });
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
        debug_assert_eq!(cells.len(), puzzle.num_categories() * labels_per_category);
        Grid {
            cells,
            puzzle,
            steps: Default::default(),
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

    #[must_use]
    fn set_impl(&mut self, label1: Label, label2: Label, val: Cell) -> Option<bool> {
        debug_assert_ne!(val, Cell::Empty);
        let c = self.at(label1, label2);
        match *c {
            Cell::Empty => {
                // info!(
                //     "  {} : {} & {}\n",
                //     if val == Cell::Yes { "✔" } else { "❌" },
                //     self.puzzle.lookup_label(label1),
                //     self.puzzle.lookup_label(label2),
                // );
                *self.at_mut(label1, label2) = val;
                Some(true)
            }
            _ => {
                if val == *c {
                    Some(false)
                } else {
                    // error!(
                    //     "CONTRADICTION: {} | {} => {:?}",
                    //     self.puzzle.lookup_label(label1),
                    //     self.puzzle.lookup_label(label2),
                    //     val
                    // );
                    None
                }
            }
        }
    }

    /// Set the cell `(label1, label2)` in the grid to `val`.
    /// Returns `None` if the attempt to set was contradictory,
    /// otherwise returns `Some(changed)` where `changed` is true iff
    /// the cell was changed from its initial value and calls the callback.
    #[must_use]
    pub fn set(&mut self, label1: Label, label2: Label, val: Cell) -> Option<bool> {
        match self.set_impl(label1, label2, val) {
            Some(true) => {
                self.steps.push(Step::new(String::new()));
                Some(true)
            }
            res => res,
        }
    }

    /// Call `set` but if it succeeds also call the `callback`.
    /// This can be used for logging information if the set goes through.
    #[must_use]
    pub fn set_with_callback<CB: FnOnce() -> String>(
        &mut self,
        label1: Label,
        label2: Label,
        val: Cell,
        callback: CB,
    ) -> Option<bool> {
        match self.set_impl(label1, label2, val) {
            Some(true) => {
                self.steps.push(Step::new(callback()));
                Some(true)
            }
            res => res,
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
                Some((Label::new(Category(c1), l1), Label::new(Category(c2), l2)))
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

        // List of rules to attempt to execute.
        let rules: &[Box<dyn Rule>] = &[
            Box::new(ElimOthers {}),
            Box::new(OnlyEmpty {}),
            Box::new(Transitivity {}),
            Box::new(NoByProxy {}),
        ];

        // Run the rules in a loop until we hit a fixed point or a contradictory condition.
        // Hopefully that's a solution.
        while changed {
            changed = false;
            for constraint in self.puzzle.constraints() {
                changed |= constraint.apply(&mut self.grid, self.puzzle)?;
            }

            for rule in rules {
                changed |= rule.apply(&mut self.grid, self.puzzle)?;
            }
        }
        Some(self.solution())
    }

    /// Create a `Solution` from the current puzzle grid.
    fn solution(self) -> Solution<'p> {
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
            steps: self.grid.steps,
        }
    }
}

pub fn solve(puzzle: &Puzzle) -> Option<Solution> {
    let solver = Solver::new(puzzle);
    solver.solve()
}
