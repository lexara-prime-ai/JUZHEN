use std::fs;

#[derive(Debug)]
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<Vec<f64>>,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        let data: Vec<Vec<f64>> = vec![vec![0.0; cols]; rows];
        Self { rows, cols, data }
    }

    pub fn from_file(path: &str) -> Self {
        let content: String = fs::read_to_string(path).unwrap_or_else(|e| panic!("{e}"));
        let mut matrix: Vec<Vec<f64>> = Vec::new();
        for rows in content.lines() {
            let mut row: Vec<f64> = Vec::new();
            let entries: Vec<&str> = rows.split_whitespace().collect();

            for ent in entries {
                row.push(ent.parse::<f64>().unwrap());
            }
            matrix.push(row);
        }

        let r: usize = matrix.len();
        let c: usize = matrix[0].len();
        Self {
            rows: r,
            cols: c,
            data: matrix,
        }
    }

    pub fn from_string(input: &str) -> Self {
        let mut data: Vec<Vec<f64>> = Vec::new();
        let rows: Vec<&str> = input.split(';').collect();

        for row in rows {
            let entries: Vec<&str> = row.split_whitespace().collect();
            let mut tmp_row: Vec<f64> = Vec::new();

            for ent in entries {
                tmp_row.push(ent.parse::<f64>().expect("FAILED TO PARSE_FLOAT"));
            }
            data.push(tmp_row);
        }

        let n_r: usize = data.len();
        let n_c: usize = data[0].len();
        Self {
            rows: n_r,
            cols: n_c,
            data,
        }
    }

    pub fn copy(&self) -> Self {
        let mut data: Vec<Vec<f64>> = Vec::new();

        for row in &self.data {
            // Can use .to_owned()
            data.push(row.to_vec());
        }
        Self {
            rows: self.rows,
            cols: self.cols,
            data,
        }
    }

    pub fn print(&self) {
        self.data.iter().for_each(|v| println!("{v:?}"));
        println!();
    }

    pub fn identity(&mut self) {
        if self.rows != self.cols {
            panic!("Not a square matrix!")
        }

        for r in 0..self.rows {
            self.data[r][r] = 1.0;
        }
    }

    pub fn apply(&mut self, f: impl Fn(f64) -> f64) {
        self.data = self
            .data
            .iter()
            .map(|v| v.iter().map(|x| f(*x)).collect())
            .collect();
    }

    // Alternatives to the apply fn
    pub fn add(&mut self, m: Matrix) -> Self {
        if self.rows != m.rows || self.cols != m.cols {
            panic!("Matices must be of the same size!");
        }

        let mut sum: Matrix = Matrix::new(self.rows, self.cols);

        for i in 0..self.rows {
            for j in 0..self.cols {
                sum.data[i][j] = self.data[i][j] + m.data[i][j];
            }
        }
        sum
    }

    pub fn subtract(&mut self, m: Matrix) -> Self {
        if self.rows != m.rows || self.cols != m.cols {
            panic!("Matices must be of the same size!");
        }

        let mut diff: Matrix = Matrix::new(self.rows, self.cols);

        for i in 0..self.rows {
            for j in 0..self.cols {
                diff.data[i][j] = self.data[i][j] - m.data[i][j];
            }
        }
        diff
    }

    pub fn dot(&self, m: Matrix) -> Self {
        if self.rows != m.cols || self.cols != m.rows {
            panic!(
                "Dimensions not matched. M1 is {} by {}, M2 is {} by {}",
                self.rows, self.cols, m.rows, m.cols
            );
        }

        let mut dp = Matrix::new(self.rows, m.cols);
        for i in 0..self.rows {
            for j in 0..m.rows {
                let mut sum = 0.0;
                for k in 0..m.rows {
                    sum += self.data[i][k] * m.data[k][j];
                }
                dp.data[i][j] = sum;
            }
        }
        dp
    }

    pub fn rref(&mut self) {
        if self.data[0][0] == 0.0 {
            swap_rows(self, 0);
        }

        let mut lead: usize = 0;
        let rows: usize = self.rows;

        while lead < rows {
            for r in 0..rows {
                let div: f64 = self.data[lead][lead];
                let mult: f64 = self.data[r][lead] / div;

                if r == lead {
                    self.data[lead] = self.data[lead].iter().map(|entry| entry / div).collect();
                } else {
                    for c in 0..self.cols {
                        self.data[r][c] -= self.data[lead][c] * mult;
                    }
                }
            }
            lead += 1;
        }
        // Handle situations where values are extremely close to 0 or another number
        correct(self);
    }

    pub fn cofactor(&self, expanded_row: usize, j: usize) -> f64 {
        // cut -> The new matrix that will be created
        let mut cut: Vec<Vec<f64>> = Vec::new();
        // Creates a smaller matrix by deleting the expanded row
        for r in 0..self.rows {
            if r == expanded_row {
                continue;
            }

            let mut v: Vec<f64> = Vec::new();
            for c in 0..self.cols {
                if c == j {
                    continue;
                }
                v.push(self.data[r][c]);
            }
            cut.push(v);
        }
        let n_r: usize = cut.len();
        let n_c: usize = cut[0].len();
        // minor -> The DETERMINANT
        let minor: f64 = Matrix {
            rows: n_r,
            cols: n_c,
            data: cut,
        }
        .det();
        let base: i32 = -1;

        minor * f64::from(base.pow((expanded_row + j) as u32))
    }

    pub fn det(&self) -> f64 {
        // Finding the determinant of a matrix is a RECUSSIVE function
        // With the COFACTORS
        // COFACTORS require the DETERMINANT and the DETERMINANT requires the COFACTOR
        if self.rows != self.cols {
            panic!("Determinant requires matrix to be a square. Input matrix was {self:?}");
        }

        if self.rows == 2 && self.cols == 2 {
            self.data[0][0] * self.data[1][1] - self.data[0][1] * self.data[1][0]
        } else {
            let row: usize = 1;
            let mut det = 0.0;

            // Expand upon the 2nd row
            for j in 0..self.data[row].len() {
                // Multiply by the element at the matrix
                det += self.cofactor(row, j) * self.data[row][j];
            }

            det
        }
    }

    pub fn transpose(&self) -> Self {
        let mut t: Matrix = Matrix::new(self.cols, self.rows);

        for i in 0..self.rows {
            for j in 0..self.cols {
                t.data[j][i] = self.data[i][j];
            }
        }
        t
    }

    pub fn inverse(&self) -> Self {
        let d: f64 = self.det();

        if d == 0.0 {
            panic!("Determinant is 0 so there can be no inverse!");
        }

        let mut inv: Matrix = Matrix::new(self.rows, self.cols);

        for row in 0..self.rows {
            for col in 0..self.cols {
                inv.data[row][col] = self.cofactor(row, col);
            }
        }

        correct(&mut inv);

        inv = inv.transpose();
        inv.apply(|x| x / d);
        inv
    }
}

fn swap_rows(m: &mut Matrix, row: usize) {
    let mut n_r: usize = 0;

    for r in 0..m.rows {
        if m.data[r][0] > 0.0 {
            n_r = r;
            break;
        }
    }

    let temp: Vec<f64> = m.data[row].clone();
    m.data[row] = m.data[n_r].clone();
    m.data[n_r] = temp;
}

fn correct(m: &mut Matrix) {
    for row in 0..m.rows {
        for col in 0..m.cols {
            let elem = m.data[row][col];

            if elem == -0.0 {
                m.data[row][col] = 0.0;
            }

            let floored = elem.floor();
            // Todo -> Clean up
            if elem - floored > 0.9999999999 {
                m.data[row][col] = elem.round();
            }

            if elem > 0.0 && elem < 0.0000001 {
                m.data[row][col] = 0.0;
            }

            if elem < 0.0 && elem > -0.0000001 {
                m.data[row][col] = 0.0;
            }
        }
    }
}
