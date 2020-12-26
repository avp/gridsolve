use crate::constraint::Constraint;
use bimap::BiMap;
use snafu::Snafu;
use std::fs::File;
use std::io::BufRead;
use std::path::PathBuf;

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

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Invalid label name: {}", name))]
    InvalidLabelName { name: String },
    #[snafu(display("Missing [Categories] marker"))]
    MissingCategories,
    #[snafu(display("Missing [Clues] marker"))]
    MissingClues,
    #[snafu(display("Invalid Clue: {}", clue))]
    InvalidClue { clue: String },
    #[snafu(display(
        "Invalid number of labels in category \"{}\", expected {} but found {}",
        category,
        expected,
        found,
    ))]
    InvalidLabelCount {
        category: String,
        expected: usize,
        found: usize,
    },
    #[snafu(display("Category not found: {}", name))]
    CategoryNotFound { name: String },
    #[snafu(display("Label not found: {}", name))]
    LabelNotFound { name: String },
    #[snafu(context(false))]
    Io { source: std::io::Error },
    #[snafu(context(false))]
    InvalidInteger { source: std::num::ParseIntError },
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

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
    /// Create a new empty Puzzle.
    pub fn new() -> Self {
        Default::default()
    }

    /// Parse a puzzle file from `path` and return the resultant Puzzle if the file
    /// was a valid puzzle file.
    pub fn from_file(path: &PathBuf) -> Result<Self> {
        let file = File::open(path)?;
        let mut lines = std::io::BufReader::new(file).lines();
        let mut puzzle = Self::new();
        loop {
            match lines.next() {
                Some(Ok(line)) => {
                    if line.trim() == "[Categories]" {
                        break;
                    }
                }
                _ => {
                    return Err(Error::MissingCategories);
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
                    return Err(Error::MissingClues);
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
                        return Err(Error::MissingClues);
                    }
                };
            }
            puzzle.add_category(&category_name, &labels)?;
        }

        for line in lines {
            let line = line?;
            let parts = line
                .trim()
                .split(',')
                .skip(1)
                .map(|s| s.trim())
                .collect::<Vec<&str>>();
            if parts.len() < 2 {
                return Err(Error::InvalidClue { clue: line });
            }
            let constraint = match parts[0] {
                "yes" => {
                    if parts.len() < 3 {
                        return Err(Error::InvalidClue { clue: line });
                    }
                    Constraint::Yes(
                        puzzle.label(parts[1])?,
                        puzzle.label(parts[2])?,
                    )
                }
                "no" => {
                    if parts.len() < 3 {
                        return Err(Error::InvalidClue { clue: line });
                    }
                    Constraint::No(
                        puzzle.label(parts[1])?,
                        puzzle.label(parts[2])?,
                    )
                }
                "after" => {
                    if parts.len() < 4 {
                        return Err(Error::InvalidClue { clue: line });
                    }
                    Constraint::After(
                        puzzle.label(parts[1])?,
                        puzzle.category(parts[2])?,
                        puzzle.label(parts[3])?,
                    )
                }
                "afteratleast" => {
                    if parts.len() < 4 {
                        return Err(Error::InvalidClue { clue: line });
                    }
                    let n: usize = parts[4].parse()?;
                    if n > puzzle.labels_per_category() - 1 {
                        return Err(Error::InvalidClue { clue: line });
                    }
                    Constraint::AfterAtLeast(
                        puzzle.label(parts[1])?,
                        puzzle.category(parts[2])?,
                        puzzle.label(parts[3])?,
                        n,
                    )
                }
                "afterexactly" => {
                    if parts.len() < 4 {
                        return Err(Error::InvalidClue { clue: line });
                    }
                    let n: usize = parts[4].parse()?;
                    if n > puzzle.labels_per_category() - 1 {
                        return Err(Error::InvalidClue { clue: line });
                    }
                    Constraint::AfterExactly(
                        puzzle.label(parts[1])?,
                        puzzle.category(parts[2])?,
                        puzzle.label(parts[3])?,
                        n,
                    )
                }
                "or" => {
                    if parts.len() < 4 {
                        return Err(Error::InvalidClue { clue: line });
                    }
                    Constraint::Or(
                        puzzle.label(parts[1])?,
                        puzzle.label(parts[2])?,
                        puzzle.label(parts[3])?,
                    )
                }
                "xor" => {
                    if parts.len() < 4 {
                        return Err(Error::InvalidClue { clue: line });
                    }
                    Constraint::Xor(
                        puzzle.label(parts[1])?,
                        puzzle.label(parts[2])?,
                        puzzle.label(parts[3])?,
                    )
                }
                "twobytwo" => {
                    if parts.len() < 5 {
                        return Err(Error::InvalidClue { clue: line });
                    }
                    Constraint::TwoByTwo(
                        puzzle.label(parts[1])?,
                        puzzle.label(parts[2])?,
                        puzzle.label(parts[3])?,
                        puzzle.label(parts[4])?,
                    )
                }
                "exactlyone" => {
                    if parts.len() < 5 || parts.len() % 2 != 1 {
                        return Err(Error::InvalidClue { clue: line });
                    }
                    let mut constraints = vec![];
                    for i in 0..parts.len() / 2 {
                        constraints.push((
                            puzzle.label(parts[i * 2 + 1])?,
                            puzzle.label(parts[i * 2 + 2])?,
                        ));
                    }
                    Constraint::ExactlyOne(constraints)
                }
                _ => return Err(Error::InvalidClue { clue: line }),
            };
            puzzle.add_constraint(constraint);
        }

        Ok(puzzle)
    }

    pub fn add_category<S: AsRef<str>>(
        &mut self,
        cat_name: &str,
        label_names: &[S],
    ) -> Result<Category> {
        if !self.category_map.is_empty()
            && label_names.len() != self.labels_per_category()
        {
            return Err(Error::InvalidLabelCount {
                category: cat_name.to_string(),
                expected: self.labels_per_category(),
                found: label_names.len(),
            });
        }
        let category = Category(self.category_map.len());
        for (i, name) in label_names.iter().enumerate() {
            if self.label(name.as_ref()).is_ok() {
                return Err(Error::InvalidLabelName {
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

    pub fn category(&self, name: &str) -> Result<Category> {
        let result = self.category_map.get_by_right(&name.to_string()).copied();
        match &result {
            None => Err(Error::CategoryNotFound {
                name: name.to_string(),
            }),
            &Some(category) => Ok(category),
        }
    }

    pub fn lookup_label(&self, label: Label) -> &str {
        self.label_map.get_by_left(&label).unwrap()
    }

    pub fn label(&self, name: &str) -> Result<Label> {
        let result = self.label_map.get_by_right(&name.to_string()).copied();
        match &result {
            None => Err(Error::LabelNotFound {
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

    pub fn labels_per_category(&self) -> usize {
        debug_assert!(!self.category_map.is_empty());
        self.label_map.len() / self.num_categories()
    }

    pub fn constraints(&self) -> &[Constraint] {
        &self.constraints
    }
}
