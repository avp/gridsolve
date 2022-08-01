use crate::constraint::{Constraint, ConstraintKind};
use anyhow::{Context, Result};
use bimap::BiMap;
use std::fs::File;
use std::io::BufRead;
use std::path::Path;

/// A category index in the puzzle.
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct Category(pub usize);

/// A label in the puzzle in a specific category.
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct Label {
    /// Category to which this label belongs.
    pub category: Category,

    /// Index of the label within `category`.
    pub label: usize,
}

impl Label {
    pub fn new(category: Category, label: usize) -> Label {
        Label { category, label }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum PuzzleError {
    #[error("Invalid label name: {}", name)]
    InvalidLabelName { name: String },
    #[error("Missing [Categories] marker")]
    MissingCategories,
    #[error("Missing [Clues] marker")]
    MissingClues,
    #[error("Invalid Clue: {}", clue)]
    InvalidClue { clue: String },
    #[error(
        "Invalid number of labels in category \"{}\", expected {} but found {}",
        category,
        expected,
        found
    )]
    InvalidLabelCount {
        category: String,
        expected: usize,
        found: usize,
    },
    #[error("Category not found: {}", name)]
    CategoryNotFound { name: String },
    #[error("Label not found: {}", name)]
    LabelNotFound { name: String },
    #[error(transparent)]
    Io {
        #[from]
        source: std::io::Error,
    },
    #[error(transparent)]
    InvalidInteger {
        #[from]
        source: anyhow::Error,
    },
}

/// The actual puzzle to be solved.
#[derive(Debug, Default)]
pub struct Puzzle {
    /// Maps from the category index to the name of the category.
    category_map: BiMap<Category, String>,

    /// Maps from the label to the name of the label.
    label_map: BiMap<Label, String>,

    /// All the constraints which arise from the clues in the puzzle.
    constraints: Vec<Constraint>,
}

impl Puzzle {
    /// Parse a puzzle file from `path` and return the resultant Puzzle
    /// if the file was a valid puzzle file.
    pub fn from_file(path: &Path) -> Result<Self, PuzzleError> {
        let file = File::open(path)?;
        let mut lines = std::io::BufReader::new(file).lines();
        let mut puzzle = Self::default();
        loop {
            match lines.next() {
                Some(Ok(line)) => {
                    if line.trim() == "[Categories]" {
                        break;
                    }
                }
                _ => {
                    return Err(PuzzleError::MissingCategories);
                }
            };
        }

        loop {
            let category_name = match lines.next() {
                Some(Ok(line)) => {
                    if line.trim() == "[Clues]" {
                        break;
                    }
                    line
                }
                _ => {
                    return Err(PuzzleError::MissingClues);
                }
            };
            let mut labels = vec![];
            loop {
                match lines.next() {
                    Some(Ok(line)) => {
                        if line.trim().is_empty() {
                            break;
                        }
                        labels.push(line);
                    }
                    _ => {
                        return Err(PuzzleError::MissingClues);
                    }
                };
            }
            puzzle.add_category(&category_name, &labels)?;
        }

        for (line_number, line) in lines.enumerate() {
            let line = line?;
            let parts_vec = line
                .trim()
                .split(',')
                .map(|s| s.trim())
                .collect::<Vec<&str>>();
            if parts_vec.len() < 3 {
                return Err(PuzzleError::InvalidClue { clue: line });
            }
            let (name, parts) = parts_vec.as_slice().split_first().unwrap();
            let constraint = match parts[0] {
                "yes" => {
                    if parts.len() < 3 {
                        return Err(PuzzleError::InvalidClue { clue: line });
                    }
                    ConstraintKind::Yes(puzzle.label(parts[1])?, puzzle.label(parts[2])?)
                }
                "no" => {
                    if parts.len() < 3 {
                        return Err(PuzzleError::InvalidClue { clue: line });
                    }
                    ConstraintKind::No(puzzle.label(parts[1])?, puzzle.label(parts[2])?)
                }
                "after" => {
                    if parts.len() < 4 {
                        return Err(PuzzleError::InvalidClue { clue: line });
                    }
                    ConstraintKind::After(
                        puzzle.label(parts[1])?,
                        puzzle.category(parts[2])?,
                        puzzle.label(parts[3])?,
                    )
                }
                "afteratleast" => {
                    if parts.len() < 4 {
                        return Err(PuzzleError::InvalidClue { clue: line });
                    }
                    let n: usize = parts[4]
                        .parse()
                        .with_context(|| format!("in line {}", line_number))
                        .map_err(|e| PuzzleError::InvalidInteger { source: e })?;
                    if n > puzzle.labels_per_category() - 1 {
                        return Err(PuzzleError::InvalidClue { clue: line });
                    }
                    ConstraintKind::AfterAtLeast(
                        puzzle.label(parts[1])?,
                        puzzle.category(parts[2])?,
                        puzzle.label(parts[3])?,
                        n,
                    )
                }
                "afterexactly" => {
                    if parts.len() < 4 {
                        return Err(PuzzleError::InvalidClue { clue: line });
                    }
                    let n: usize = parts[4]
                        .parse()
                        .with_context(|| format!("in line {}", line_number))
                        .map_err(|e| PuzzleError::InvalidInteger { source: e })?;
                    if n > puzzle.labels_per_category() - 1 {
                        return Err(PuzzleError::InvalidClue { clue: line });
                    }
                    ConstraintKind::AfterExactly(
                        puzzle.label(parts[1])?,
                        puzzle.category(parts[2])?,
                        puzzle.label(parts[3])?,
                        n,
                    )
                }
                "or" => {
                    if parts.len() < 4 {
                        return Err(PuzzleError::InvalidClue { clue: line });
                    }
                    ConstraintKind::Or(
                        puzzle.label(parts[1])?,
                        puzzle.label(parts[2])?,
                        puzzle.label(parts[3])?,
                    )
                }
                "xor" => {
                    if parts.len() < 4 {
                        return Err(PuzzleError::InvalidClue { clue: line });
                    }
                    ConstraintKind::Xor(
                        puzzle.label(parts[1])?,
                        puzzle.label(parts[2])?,
                        puzzle.label(parts[3])?,
                    )
                }
                "twobytwo" => {
                    if parts.len() < 5 {
                        return Err(PuzzleError::InvalidClue { clue: line });
                    }
                    ConstraintKind::TwoByTwo(
                        puzzle.label(parts[1])?,
                        puzzle.label(parts[2])?,
                        puzzle.label(parts[3])?,
                        puzzle.label(parts[4])?,
                    )
                }
                "exactlyone" => {
                    if parts.len() < 5 || parts.len() % 2 != 1 {
                        return Err(PuzzleError::InvalidClue { clue: line });
                    }
                    let mut constraints = vec![];
                    for i in 0..parts.len() / 2 {
                        constraints.push((
                            puzzle.label(parts[i * 2 + 1])?,
                            puzzle.label(parts[i * 2 + 2])?,
                        ));
                    }
                    ConstraintKind::ExactlyOne(constraints)
                }
                _ => return Err(PuzzleError::InvalidClue { clue: line }),
            };
            puzzle.add_constraint(Constraint {
                kind: constraint,
                name: name.to_string(),
            });
        }

        Ok(puzzle)
    }

    pub fn add_category<S: AsRef<str>>(
        &mut self,
        cat_name: &str,
        label_names: &[S],
    ) -> Result<Category, PuzzleError> {
        if !self.category_map.is_empty() && label_names.len() != self.labels_per_category() {
            return Err(PuzzleError::InvalidLabelCount {
                category: cat_name.to_string(),
                expected: self.labels_per_category(),
                found: label_names.len(),
            });
        }
        let category = Category(self.category_map.len());
        for (i, name) in label_names.iter().enumerate() {
            if self.label(name.as_ref()).is_ok() {
                return Err(PuzzleError::InvalidLabelName {
                    name: name.as_ref().to_string(),
                });
            }
            let label = Label { category, label: i };
            self.label_map.insert(label, name.as_ref().to_string());
        }
        self.category_map.insert(category, cat_name.to_string());
        Ok(category)
    }

    pub fn lookup_category(&self, category: Category) -> &str {
        self.category_map.get_by_left(&category).unwrap()
    }

    pub fn category(&self, name: &str) -> Result<Category, PuzzleError> {
        let result = self.category_map.get_by_right(&name.to_string()).copied();
        match &result {
            None => Err(PuzzleError::CategoryNotFound {
                name: name.to_string(),
            }),
            &Some(category) => Ok(category),
        }
    }

    pub fn lookup_label(&self, label: Label) -> &str {
        self.label_map.get_by_left(&label).unwrap()
    }

    pub fn label(&self, name: &str) -> Result<Label, PuzzleError> {
        let result = self.label_map.get_by_right(&name.to_string()).copied();
        match &result {
            None => Err(PuzzleError::LabelNotFound {
                name: name.to_string(),
            }),
            &Some(label) => Ok(label),
        }
    }

    pub fn add_constraint(&mut self, constraint: Constraint) {
        self.constraints.push(constraint);
    }

    pub fn num_categories(&self) -> usize {
        self.category_map.len()
    }

    pub fn categories(&self) -> impl Iterator<Item = Category> {
        (0..self.num_categories()).map(Category)
    }

    pub fn labels_per_category(&self) -> usize {
        debug_assert!(!self.category_map.is_empty());
        self.label_map.len() / self.num_categories()
    }

    pub fn constraints(&self) -> &[Constraint] {
        &self.constraints
    }
}
