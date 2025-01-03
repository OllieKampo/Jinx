
/// TODO: Could we do this as a decorator macro?
macro_rules! assert_same_size {
    ($self:ident, $other:ident) => {
        assert_eq!($self.rows, $other.rows);
        assert_eq!($self.cols, $other.cols);
    };
}

// TODO: How can we do dimensions as generics?
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
    pub fn apply_element_wise_operator(&mut self, operator: impl Fn(usize, usize, f64) -> f64) {
        for i in 0..self.rows {
            for j in 0..self.cols {
                let current_value = self.get(i, j);
                let new_value = operator(i, j, current_value);
                self.set(i, j, new_value);
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
        result.apply_element_wise_operator(|i, j, v| v + other.get(i, j));
        return result;
    }
}

impl std::ops::Add<f64> for Matrix {
    type Output = Matrix;

    fn add(self, scalar: f64) -> Matrix {
        let mut result = self.clone();
        result.apply_element_wise_operator(|_, _, v| v + scalar);
        return result;
    }
}

impl std::ops::Sub<&Matrix> for &Matrix {
    type Output = Matrix;

    fn sub(self, other: &Matrix) -> Matrix {
        assert_same_size!(self, other);
        let mut result = self.clone();
        result.apply_element_wise_operator(|i, j, v| v - other.get(i, j));
        return result;
    }
}

impl std::ops::Sub<f64> for Matrix {
    type Output = Matrix;

    fn sub(self, scalar: f64) -> Matrix {
        let mut result = Matrix::new(self.rows, self.cols);
        result.apply_element_wise_operator(|_, _, v| v - scalar);
        return result;
    }
}

impl std::ops::Mul<&Matrix> for &Matrix {
    type Output = Matrix;

    fn mul(self, other: &Matrix) -> Matrix {
        assert_same_size!(self, other);
        let mut result = self.clone();
        result.apply_element_wise_operator(|i, j, v| v * other.get(i, j));
        return result;
    }
}

impl std::ops::Mul<f64> for Matrix {
    type Output = Matrix;

    fn mul(self, scalar: f64) -> Matrix {
        let mut result = Matrix::new(self.rows, self.cols);
        result.apply_element_wise_operator(|_, _, v| v * scalar);
        return result;
    }
}

impl std::ops::Div<&Matrix> for &Matrix {
    type Output = Matrix;

    fn div(self, other: &Matrix) -> Matrix {
        assert_same_size!(self, other);
        let mut result = self.clone();
        result.apply_element_wise_operator(|i, j, v| v / other.get(i, j));
        return result;
    }
}

impl std::ops::Div<f64> for Matrix {
    type Output = Matrix;

    fn div(self, scalar: f64) -> Matrix {
        let mut result = Matrix::new(self.rows, self.cols);
        result.apply_element_wise_operator(|_, _, v| v / scalar);
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
