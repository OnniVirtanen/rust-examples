#[derive(PartialEq, Debug)]
struct Matrix {
    data: Vec<Vec<i64>>,
    rows: usize,
    columns: usize,
}

impl Matrix {
    pub fn new(data: Vec<Vec<i64>>, rows: usize, columns: usize) -> Self {
        return Self {
            data: data,
            rows: rows,
            columns: columns,
        };
    }

    pub fn matmul(a: Matrix, b: Matrix) -> Self {
        let n = a.rows;
        let m = a.columns;
        let p = b.columns;
        let mut c = Matrix::new(vec![vec![0; p]; n], n, p);

        for i in 0..n {
            for j in 0..p {
                let mut sum = 0;
                for k in 0..m {
                    sum += a.data[i][k] * b.data[k][j];
                }
                c.data[i][j] = sum;
            }
        }

        return c;
    }

    pub fn get_dimensions(&self) -> (usize, usize) {
        return (self.rows, self.columns);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_multiplication_1x1_output() {
        let a = Matrix::new(vec![vec![2, 5, 6]], 1, 3);
        let b = Matrix::new(vec![vec![3], vec![4], vec![-5]], 3, 1);

        let ab = Matrix::matmul(a, b);

        assert_eq!(ab.data[0][0], -4);
    }

    #[test]
    fn test_matrix_multiplications_3x3_output() {
        let a = Matrix::new(vec![vec![2, 5, 6]], 1, 3);
        let b = Matrix::new(vec![vec![3], vec![4], vec![-5]], 3, 1);
        let expected = Matrix::new(vec![vec![6, 15, 18], vec![8, 20, 24], vec![-10, -25, -30]], 3, 3);
        
        let ba = Matrix::matmul(b, a);
        
        assert_eq!(ba, expected);
    }

    #[test]
    fn test_matrix_multiplication_2x4_output() {
        let a = Matrix::new(vec![vec![1, 4, -2], vec![3, 5, -6]], 2, 3);
        let b = Matrix::new( vec![vec![5, 2, 8, -1], vec![3, 6, 4, 5], vec![-2, 9, 7, -3]], 3, 4);
        let expected = Matrix::new(vec![vec![21, 8, 10, 25], vec![42, -18, 2, 40]], 2, 4);

        let ab = Matrix::matmul(a, b);

        assert_eq!(expected, ab);
    }
    
    #[test]
    fn test_matrix_dimensions() {
        let a = Matrix::new(vec![vec![2, 5, 6]], 1, 3);
        let b = Matrix::new(vec![vec![3], vec![4], vec![-5]], 3, 1);
        
        assert_eq!(a.get_dimensions(), (1, 3));
        assert_eq!(b.get_dimensions(), (3, 1));
    }
}
