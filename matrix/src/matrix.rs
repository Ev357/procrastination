use std::{
    array,
    ops::{Add, Mul},
};

#[derive(Debug)]
pub struct Matrix<
    T: Add<Output = T> + Mul<Output = T> + Copy + Default,
    const ROWS: usize,
    const COLS: usize,
> {
    pub data: [[T; COLS]; ROWS],
}

impl<T: Add<Output = T> + Mul<Output = T> + Copy + Default, const ROWS: usize, const COLS: usize>
    Matrix<T, ROWS, COLS>
{
    pub fn new(data: [[T; COLS]; ROWS]) -> Self {
        Self { data }
    }

    pub fn multiply<const N: usize>(&self, matrix: &Matrix<T, COLS, N>) -> Matrix<T, ROWS, N> {
        let data = array::from_fn(|i| {
            array::from_fn(|j| {
                (0..COLS)
                    .map(|k| self.data[i][k] * matrix.data[k][j])
                    .fold(T::default(), |acc, val| acc + val)
            })
        });

        Matrix { data }
    }
}
