//
#[derive(Debug)]
pub struct SquareMatrix {
    data: Vec<f64>,
    len: usize,
}

impl SquareMatrix {
    pub fn new(len: usize) -> Self {
        Self {
            data: vec![0.0; len * len],
            len,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub unsafe fn get_unchecked(&self, row: usize, col: usize) -> &f64 {
        let idx = row * self.len + col;
        unsafe { self.data.get_unchecked(idx) }
    }

    pub unsafe fn get_unchecked_mut(&mut self, row: usize, col: usize) -> &mut f64 {
        let idx = row * self.len + col;
        unsafe { self.data.get_unchecked_mut(idx) }
    }
}
