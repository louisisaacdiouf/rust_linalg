use colored::Colorize;
use core::fmt;
use std::ops::{Add, Sub, Mul};

#[derive(Clone)]
pub struct Matrix<T> {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<Vec<T>>,
}

impl<T> Matrix<T>
where
    T: Clone + Default + From<u8> + Mul + std::iter::Sum<<T as Mul>::Output>,
    Vec<T>: FromIterator<<T as Mul>::Output>
{
    pub fn new(rows: Vec<Vec<T>>) -> Self {
        if rows.iter().any(|r| r.len() != rows[0].len()) {
            panic!("{}", "All rows must have same length.".red());
        }

        Matrix {
            rows: rows.len(),
            cols: rows[0].len(),
            data: rows,
        }
    }

    // Verify if both matrices have same dims
    fn check_same_dims(&self, rhs: &Self) {
        // Check valid dims
        self.check_valid_dims();
        rhs.check_valid_dims();

        if self.rows != rhs.rows || self.cols != rhs.cols {
            panic!("{}", "Matrices must have same dims.".red());
        }
    }

    // Verify if provided dims are valid.
    fn check_valid_dims(&self) {
        if self.data.iter().any(|r| r.len() != self.data[0].len())
            || self.data.len() != self.rows
            || self.data.iter().any(|r| r.len() != self.cols)
        {
            panic!("{}", "Matrices must have valid dims.".red());
        }
    }

    pub fn zeros(n: usize, p: usize) -> Self {
        Matrix::new(vec![vec![T::from(0u8); p]; n])
    }

    pub fn ones(n: usize, p: usize) -> Self {
        Matrix::new(vec![vec![T::from(1u8); p]; n])
    }

    pub fn row(&self, i: usize) -> Vec<T> {
        if i >= self.rows {
            panic!("{}", "Invalid row index requested.".red());
        }

        self.data[i].clone()
    }

    pub fn col(&self, i: usize) -> Vec<T> {
        if i >= self.cols {
            panic!("{}", "Invalid column index requested.".red());
        }

        self.data.iter().map(|r| r[i].clone()).collect()
    }

    pub fn dot(&self, rhs: &Self) -> Self {
        if self.cols != rhs.rows {
            panic!("{}", "Incompatible matrices dims for dot product.".red());
        }

        let mut res: Matrix<T> = Matrix::ones(self.rows, rhs.cols);

        for n in 0..self.rows {
            for p in 0..rhs.cols {
                res.data[n][p] = self
                    .row(n)
                    .iter()
                    .zip(rhs.col(p))
                    .map(|(a, b)| a.clone() * b.clone())
                    .sum()
            }
        }
        res
    }

    pub fn dot_scalar(&self, a: T) -> Self {
        let mut res: Matrix<T> = self.clone();

        res.data  = res.data.into_iter().map(|r| r.into_iter().map(|v| a.clone() * v).collect()).collect();
        res
    }

    pub fn transpose(&self) -> Self {
        self.check_valid_dims();

        let mut  data: Vec<Vec<T>> = vec![];

        for i in 0..self.cols {
            data.push(self.col(i));
        }

        Matrix::new(data)
    }
}

impl<T: std::ops::Add> Add for Matrix<T>
where
    T: Add<Output = T>
        + Clone
        + Default
        + From<u8>
        + Mul
        + std::iter::Sum<<T as Mul>::Output>,
    Vec<T>: FromIterator<<T as Add>::Output>,
    Vec<T>: FromIterator<<T as Mul>::Output>
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.check_same_dims(&rhs);

        Matrix::new(
            self.data
                .into_iter()
                .zip(rhs.data)
                .map(|(a, b)| a.into_iter().zip(b).map(|(x, y)| x + y).collect::<Vec<T>>())
                .collect::<Vec<Vec<T>>>(),
        )
    }
}

impl<T: std::ops::Sub> Sub for Matrix<T>
where
    T: Add<Output = T>
        + Clone
        + Default
        + From<u8>
        + Mul
        + std::iter::Sum<<T as Mul>::Output>,
    Vec<T>: FromIterator<<T as Sub>::Output>,
    Vec<T>: FromIterator<<T as Mul>::Output>
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.check_same_dims(&rhs);

        Matrix::new(
            self.data
                .into_iter()
                .zip(rhs.data)
                .map(|(a, b)| a.into_iter().zip(b).map(|(x, y)| x - y).collect::<Vec<T>>())
                .collect::<Vec<Vec<T>>>(),
        )
    }
}

impl<T: std::fmt::Debug> fmt::Display for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output: Vec<String> = vec![];
        for r in self.data.iter() {
            output.push(format!("{:?}", r));
        }

        write!(f, "{}", output.join("\n"))
    }
}
