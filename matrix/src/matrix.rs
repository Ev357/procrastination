use core::{
    fmt::{self, Display},
    iter::Iterator,
};
use std::{
    array,
    ops::{Add, Mul},
};

#[derive(Debug)]
pub struct Matrix<
    T: Add<Output = T> + Mul<Output = T> + Clone + Default,
    const ROWS: usize,
    const COLS: usize,
> {
    pub data: [[T; COLS]; ROWS],
}

impl<T: Add<Output = T> + Mul<Output = T> + Clone + Default, const ROWS: usize, const COLS: usize>
    Matrix<T, ROWS, COLS>
{
    pub fn new(data: [[T; COLS]; ROWS]) -> Self {
        Self { data }
    }

    pub fn multiply<const N: usize>(&self, matrix: &Matrix<T, COLS, N>) -> Matrix<T, ROWS, N> {
        let data = array::from_fn(|i| {
            array::from_fn(|j| {
                (0..COLS)
                    .map(|k| self.data[i][k].clone() * matrix.data[k][j].clone())
                    .fold(T::default(), |acc, val| acc + val)
            })
        });

        Matrix { data }
    }
}

impl<
    T: Add<Output = T> + Mul<Output = T> + Clone + Default + Display,
    const ROWS: usize,
    const COLS: usize,
> Display for Matrix<T, ROWS, COLS>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::new();

        for (index, row) in self.data.iter().enumerate() {
            output.push_str("| ");

            for (index, value) in row.iter().enumerate() {
                output.push_str(&format!("{}", value));

                if index != row.len() - 1 {
                    output.push_str(", ")
                }
            }

            output.push_str(" |");

            if index != self.data.len() - 1 {
                output.push('\n');
            }
        }

        write!(f, "{}", output)?;

        Ok(())
    }
}
