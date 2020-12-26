use crate::puzzle::*;
use crate::solver::*;
use log::info;

pub trait Rule {
    /// Return whether the application altered the grid at all.
    fn apply(&self, grid: &mut Grid) -> Option<bool>;
}

/// Eliminate the other cells in the row/column of a Yes cell.
/// If we have labels (x1,x2,x3) and (y1,y2,y3),
/// then if (x1,y1) == Yes, then all other x,y pairings must be No.
pub struct ElimOthers {}

impl Rule for ElimOthers {
    fn apply(&self, grid: &mut Grid) -> Option<bool> {
        info!("Running ElimOthers...\n");
        let mut changed = false;
        for (l1, l2) in grid.cells() {
            if *grid.at(l1, l2) == Cell::Yes {
                for l in 0..grid.labels_per_category {
                    if l == l2.label {
                        continue;
                    }
                    let l3 = Label {
                        category: l2.category,
                        label: l,
                    };
                    changed |= grid.set(l1, l3, Cell::No)?;
                }
                for l in 0..grid.labels_per_category {
                    if l == l1.label {
                        continue;
                    }
                    let l3 = Label {
                        category: l1.category,
                        label: l,
                    };
                    changed |= grid.set(l3, l2, Cell::No)?;
                }
            }
        }
        Some(changed)
    }
}

/// If all cells in a given row or column are No except for a single Empty,
/// then that cell must be Yes.
pub struct OnlyEmpty {}

impl Rule for OnlyEmpty {
    fn apply(&self, grid: &mut Grid) -> Option<bool> {
        info!("Running OnlyEmpty...\n");
        let mut changed = false;
        for (l1, l2) in grid.cells() {
            if *grid.at(l1, l2) == Cell::Empty {
                let mut only = true;
                for l in 0..grid.labels_per_category {
                    if l == l2.label {
                        continue;
                    }
                    let l3 = Label {
                        category: l2.category,
                        label: l,
                    };
                    if *grid.at(l1, l3) != Cell::No {
                        only = false;
                    }
                }

                if only {
                    changed |= grid.set(l1, l2, Cell::Yes)?;
                    continue;
                }

                only = true;
                for l in 0..grid.labels_per_category {
                    if l == l1.label {
                        continue;
                    }
                    let l3 = Label {
                        category: l1.category,
                        label: l,
                    };
                    if *grid.at(l3, l2) != Cell::No {
                        only = false;
                    }
                }

                if only {
                    changed |= grid.set(l1, l2, Cell::Yes)?;
                    continue;
                }
            }
        }
        Some(changed)
    }
}

/// If (x,y) == Yes and (y,z) == Yes, then (x,z) == Yes.
/// If (x,y) == Yes and (x,z) == Yes, then (y,z) == Yes.
pub struct Transitivity {}

impl Rule for Transitivity {
    fn apply(&self, grid: &mut Grid) -> Option<bool> {
        info!("Running Transitivity...\n");
        let mut changed = false;
        for (x, y) in grid.cells() {
            if *grid.at(x, y) == Cell::Yes {
                for z in grid.labels() {
                    if *grid.at(x, z) == Cell::Yes {
                        changed |= grid.set(y, z, Cell::Yes)?;
                    }
                    if *grid.at(y, z) == Cell::Yes {
                        changed |= grid.set(x, z, Cell::Yes)?;
                    }
                }
            }
        }
        Some(changed)
    }
}

/// Suppose we have some cell (x,y) where x and y are from separate categories,
/// Cx and Cz.
/// If for every z in some other category Cz (Cz != Cx and Cz != Cy),
/// (x,z) == No and (x,y) == No, then there exists no element of Cz which can
/// match the entity that has attributes x and y.
/// Thus, there is no path from x to y via z, and we can say (x,y) == No.
pub struct NoByProxy {}

impl Rule for NoByProxy {
    fn apply(&self, grid: &mut Grid) -> Option<bool> {
        info!("Running NoByProxy...\n");
        let mut changed = false;
        for (x, y) in grid.cells() {
            let (cx, cy) = (x.category, y.category);
            if cx == cy {
                continue;
            }
            for cz in grid.categories() {
                if cz == cx || cz == cy {
                    continue;
                }
                // If for every z in cz, either (x,z) == No or (y,z) == No,
                // then there is no path to (x,y) == Yes, because they cannot
                // be reconciled.
                let mut has_path = false;
                for i in 0..grid.labels_per_category {
                    let z = Label::new(cz, i);
                    if *grid.at(x, z) != Cell::No && *grid.at(y, z) != Cell::No
                    {
                        // (x,y) == Yes is reconcilable in category cz.
                        has_path = true;
                        // Stop iteration because we can never break the path now.
                        break;
                    }
                }
                if !has_path {
                    // No path in one category, no point trying the rest.
                    changed |= grid.set(x, y, Cell::No)?;
                    break;
                }
            }
        }
        Some(changed)
    }
}
