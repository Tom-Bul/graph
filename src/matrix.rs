pub struct Matrix {
    rows: [[i16; 3]; 3],
}

impl Matrix {
    pub fn new(rows: [[i16; 3]; 3]) -> Self {
        Matrix { rows }
    }

    pub fn get(&self, row: usize, col: usize) -> Option<i16> {
        let val = self.rows.get(row)?.get(col)?;

        Some(*val)
    }

    pub fn det(&self) -> Option<i16> {
        let a = self.get(0, 0)?
            * (self.get(1, 1)? * self.get(2, 2)? - self.get(1, 2)? * self.get(2, 1)?);

        let b = self.get(0, 1)?
            * (self.get(1, 0)? * self.get(2, 2)? - self.get(2, 0)? * self.get(1, 2)?);

        let c = self.get(0, 2)?
            * (self.get(1, 0)? * self.get(2, 1)? - self.get(2, 0)? * self.get(1, 1)?);

        Some(a - b + c)
    }
}