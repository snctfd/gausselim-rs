use std::ops::Index;
use std::{fmt, ptr};
use std::str::FromStr;
use std::error::Error;

struct Matrix {
    matrix: Vec<f32>,
    rows: usize,
    cols: usize,
}

impl Matrix {
    fn swap_rows(&mut self, i: usize, j: usize) {
        let c = self.cols;
        assert!(i < self.rows);
        assert!(j < self.rows);

        if i != j {
            unsafe {
                ptr::swap_nonoverlapping(&mut self.matrix[i * c], &mut self.matrix[j * c], c);
            }
        }
    }

    fn mult_row(&mut self, row: usize, val: f32) {
        let c = self.cols;
        assert!(row < self.rows);

        // TODO: add mutable indexing for matrix
        self.matrix[row * c .. (row + 1) * c]
            .iter_mut()
            .for_each(|x| *x *= val);
    }

    // adds the values in row j to the values in row i
    fn add_row(&mut self, to: usize, from: usize) {
        let c = self.cols;
        assert!(to < self.rows);
        assert!(from < self.rows);

        // I'd like to do this by zipping the to-row with the from-row and modifying the to-row in
        // place, but it seems like that incurs the wrath of the borrow checker.
        for i in 0..c {
            self.matrix[to * c + i] += self.matrix[from * c + i]
        }
    }
}

impl Index<usize> for Matrix {
    type Output = [f32];

    // Returns the index'th row
    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.rows);
        let c = self.cols;

        &self.matrix[index * c .. (index + 1) * c]
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let r = self.rows;
        let c = self.cols;

        for i in 0..r {
            write!(f, "[ ")?;

            for j in 0..c {
                write!(f, "{} ", self[i][j])?;
            }

            write!(f, "]\n")?;
        }

        Ok(())
    }
}

// expects two integers (rows cols), followed by rows*cols real numbers.
// starts top-left, fills in row-major order
impl FromStr for Matrix {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split_whitespace();
        let r: usize = it.next().unwrap().parse()?;
        let c: usize = it.next().unwrap().parse()?;
        let mut v = Vec::with_capacity(r * c);

        for x in it {
            let n = x.parse()?;
            v.push(n)
        }

        assert_eq!(v.len(), r * c);

        Ok(Self {
            matrix: v,
            rows: r,
            cols: c
        })
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn mat() -> Matrix {
        // [ 1 2 3 ]
        // [ 4 5 6 ]
        Matrix {
            matrix: vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
            rows: 2,
            cols: 3
        }
    }

    fn mat2() -> Matrix {
        // [ 1 2 ]
        // [ 3 4 ]
        // [ 5 6 ]
        Matrix {
            matrix: vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
            rows: 3,
            cols: 2
        }
    }

    #[test]
    fn indexing() {
        let a = mat();

        assert_eq!(a[1], [4.0, 5.0, 6.0]);
        assert_eq!(a[0][2], 3.0);
    }

    #[test]
    fn string() {
        let to = "[ 1 2 3 ]\n[ 4 5 6 ]\n";
        let from = "2 3\n1 2 3\n4 5 6\n";
        let a = mat();
        let parsed = from.parse::<Matrix>().unwrap();

        assert_eq!(a.to_string(), to);
        assert_eq!(parsed[1], [4.0, 5.0, 6.0]);
        assert_eq!(parsed[0][1], 2.0);
    }

    #[test]
    fn swap_rows() {
        let a = &mut mat2();
        a.swap_rows(0, 2);

        assert_eq!(a[0], [5.0, 6.0]);
        assert_eq!(a[1], [3.0, 4.0]);
        assert_eq!(a[2], [1.0, 2.0]);
    }

    #[test]
    fn mult_row() {
        let a = &mut mat();
        a.mult_row(0, 2.5);

        assert_eq!(a[0], [2.5, 5.0, 7.5]);
        assert_eq!(a[1], [4.0, 5.0, 6.0]);
    }

    #[test]
    fn add_row() {
        let a = &mut mat2();
        a.add_row(0, 2);

        assert_eq!(a[0], [6.0, 8.0]);
        assert_eq!(a[1], [3.0, 4.0]);
        assert_eq!(a[2], [5.0, 6.0]);
    }
}
