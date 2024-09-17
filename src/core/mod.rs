pub mod cv;

#[derive(Default)]
pub enum MatKind {
    Gray,
    U8C1,
    U8C2,
    #[default]
    U8C3,
    U8C4,
}

#[derive(Default)]
pub struct Mat {
    rows: usize,
    cols: usize,
    chan: usize,
    kind: MatKind,
    data: Vec<Vec<f64>>,
}

impl Mat {
    pub fn new() -> Self {
        Mat::default()
    }

    pub fn new_with_size(w: usize, h: usize) -> Self {
        let mut mat = Mat::new();
        mat.cols = w;
        mat.rows = h;
        mat.chan = 3;
        mat.kind = MatKind::U8C3;
        mat.data = vec![vec![0.0; w]; h];
        mat
    }

    pub fn size(&self) -> (usize, usize) {
        (self.cols, self.rows)
    }

    pub fn chan(&self) -> usize {
        self.chan
    }

    pub fn data(&self) -> &Vec<Vec<f64>> {
        &self.data
    }
}
