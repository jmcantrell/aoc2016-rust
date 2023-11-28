use nalgebra::DMatrix;

use super::{Axis, Operation};

pub type Pixel = bool;

type Inner = DMatrix<Pixel>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Screen(Inner);

impl Screen {
    pub fn new(width: usize, height: usize) -> Self {
        Self(Inner::from_element(height, width, false))
    }
}

impl Screen {
    pub fn apply_rect(&mut self, width: usize, height: usize) {
        for row in 0..height {
            for column in 0..width {
                self.0[(row, column)] = true;
            }
        }
    }

    pub fn apply_rotate_row(&mut self, index: usize, count: usize) {
        let n = self.0.ncols();

        let row: Vec<_> = self
            .0
            .row(index)
            .iter()
            .cycle()
            .skip(n - count)
            .take(n)
            .cloned()
            .collect();

        for (current, next) in self.0.row_mut(index).iter_mut().zip(row.into_iter()) {
            *current = next;
        }
    }

    pub fn apply_rotate_column(&mut self, index: usize, count: usize) {
        let n = self.0.nrows();

        let column: Vec<_> = self
            .0
            .column(index)
            .iter()
            .cycle()
            .skip(n - count)
            .take(n)
            .cloned()
            .collect();

        for (current, next) in self.0.column_mut(index).iter_mut().zip(column.into_iter()) {
            *current = next;
        }
    }

    pub fn apply(&mut self, operation: &Operation) {
        match operation {
            Operation::Rect { width, height } => self.apply_rect(*width, *height),
            Operation::Rotate { axis, index, count } => match axis {
                Axis::Row => self.apply_rotate_row(*index, *count),
                Axis::Column => self.apply_rotate_column(*index, *count),
            },
        }
    }

    pub fn lit_count(&self) -> usize {
        self.0.iter().filter(|&&lit| lit).count()
    }
}

impl std::fmt::Display for Screen {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in self.0.row_iter() {
            for &lit in row.iter() {
                write!(f, "{}", if lit { '#' } else { '.' })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
