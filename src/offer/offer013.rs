struct NumMatrix {
    pub inner: Vec<Vec<i32>>,
}

#[allow(dead_code)]
impl NumMatrix {
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        let num_row = matrix.len();
        let num_col = matrix[0].len();
        let mut inner = Vec::with_capacity(num_row + 1);
        for _ in 0..num_row + 1 {
            inner.push(vec![0; num_col + 1]);
        }

        for i in 0..num_row {
            for j in 0..num_col {
                inner[i + 1][j + 1] =
                    matrix[i][j] - inner[i][j] + inner[i][j + 1] + inner[i + 1][j];
            }
        }

        println!("{:?}", inner.clone());
        Self { inner }
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let row1 = row1 as usize;
        let row2 = row2 as usize;
        let col1 = col1 as usize;
        let col2 = col2 as usize;
        return self.inner[row2 + 1][col2 + 1]
            - self.inner[row2 + 1][col1]
            - self.inner[row1][col2 + 1]
            + self.inner[row1][col1];
    }
}

#[test]
fn test1() {
    let v = vec![
        vec![1, 2, 3, 4, 5],
        vec![1, 2, 3, 4, 5],
        vec![1, 2, 3, 4, 5],
    ];

    let obj = NumMatrix::new(v);
    let result1 = obj.sum_region(1, 1, 2, 2);
    let result2 = obj.sum_region(0, 0, 2, 3);
    assert_eq!(result1, 10);
    assert_eq!(result2, 30);
}
