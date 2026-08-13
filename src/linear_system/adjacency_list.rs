use crate::linear_system::LinearSystem;

struct AdjacencyList {
    rum: Vec<Vec<(usize, f64)>>,
    bs: Vec<f64>,
    len: usize,
}

impl AdjacencyList {
    pub fn new(ls: LinearSystem) -> Self {
        todo!()
    }
}
