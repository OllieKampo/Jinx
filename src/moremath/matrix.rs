
/// TODO: Could we do this as a decorator macro?
macro_rules! assert_same_size {
    ($self:ident, $other:ident) => {
        assert_eq!($self.rows, $other.rows);
        assert_eq!($self.cols, $other.cols);
    };
}

#[derive(Clone)]
pub struct Matrix {
    pub data: Vec<f64>,
    pub rows: usize,
    pub cols: usize,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Matrix {
        Matrix {
            data: vec![0.0; rows * cols],
            rows,
            cols,
        }
    }

    pub fn get(&self, row: usize, col: usize) -> f64 {
        self.data[row * self.cols + col]
    }

    pub fn set(&mut self, row: usize, col: usize, value: f64) {
        self.data[row * self.cols + col] = value;
    }

    #[inline]
    pub fn apply_element_wise_operator(&mut self, other: &Matrix, operator: fn(&Self, &Matrix, usize, usize) -> f64) {
        assert_eq!(self.rows, other.rows);
        assert_eq!(self.cols, other.cols);
        for i in 0..self.rows {
            for j in 0..self.cols {
                let value = operator(self, other, i, j);
                self.set(i, j, value);
            }
        }
    }

    pub fn transpose(&self) -> Matrix {
        let mut result = Matrix::new(self.cols, self.rows);
        for i in 0..self.rows {
            for j in 0..self.cols {
                result.set(j, i, self.get(i, j));
            }
        }
        result
    }
}

impl std::ops::Add<&Matrix> for &Matrix {
    type Output = Matrix;

    fn add(self, other: &Matrix) -> Matrix {
        assert_same_size!(self, other);
        let mut result = self.clone();
        result.apply_element_wise_operator(other, |m, o, i, j| m.get(i, j) + o.get(i, j));
        return result;
    }
}

// impl std::ops::Add<f64> for Matrix {
//     type Output = Matrix;

//     fn add(self, scalar: f64) -> Matrix {
//         let mut result = self.clone();
//         result.apply_element_wise_operator(&self, |m, _, i, j| m.get(i, j) + scalar);
//         return result;
//     }
// }

impl std::ops::Sub<&Matrix> for &Matrix {
    type Output = Matrix;

    fn sub(self, other: &Matrix) -> Matrix {
        assert_same_size!(self, other);
        let mut result = Matrix::new(self.rows, self.cols);
        for i in 0..self.rows {
            for j in 0..self.cols {
                let value = self.get(i, j) - other.get(i, j);
                result.set(i, j, value);
            }
        }
        return result;
    }
}

impl std::ops::Sub<f64> for Matrix {
    type Output = Matrix;

    fn sub(self, scalar: f64) -> Matrix {
        let mut result = Matrix::new(self.rows, self.cols);
        for i in 0..self.rows {
            for j in 0..self.cols {
                let value = self.get(i, j) - scalar;
                result.set(i, j, value);
            }
        }
        return result;
    }
}

impl std::fmt::Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for i in 0..self.rows {
            for j in 0..self.cols {
                write!(f, "{:.2} ", self.get(i, j))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
