use minifb::{Key, Window, WindowOptions};

use crate::one;

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
    /// use 32-bit pixel.
    /// The encoding for each pixel is 0RGB:
    /// The upper 8-bits are ignored, the next 8-bits are for the red channel,
    /// the next 8-bits afterwards for the green channel, and the lower 8-bits for the blue channel.
    data: Vec<Vec<u32>>,
}

impl Mat {
    pub fn new_with_size(w: usize, h: usize) -> Self {
        let mut mat = Mat::default();
        mat.cols = w;
        mat.rows = h;
        mat.chan = 3;
        mat.kind = MatKind::U8C3;
        mat.data = vec![vec![0; w]; h];
        mat
    }
    pub fn new_with_size_rgb(w: usize, h: usize, rgb: u32) -> Self {
        let mut mat = Mat::default();
        mat.cols = w;
        mat.rows = h;
        mat.chan = 3;
        mat.kind = MatKind::U8C3;
        mat.data = vec![vec![rgb; w]; h];
        mat
    }

    pub fn from_rgb(r: u8, g: u8, b: u8) -> u32 {
        let (r, g, b) = (r as u32, g as u32, b as u32);
        (r << 16) | (g << 8) | b
    }

    pub fn to_rgb(rgb: u32) -> (u8, u8, u8) {
        (
            (rgb >> 16) as u8 & 0xff,
            (rgb >> 8) as u8 & 0xff,
            rgb as u8 & 0xff,
        )
    }

    pub fn size(&self) -> (usize, usize) {
        (self.cols, self.rows)
    }

    pub fn chan(&self) -> usize {
        self.chan
    }

    pub fn data(&self) -> &Vec<Vec<u32>> {
        &self.data
    }

    pub fn w(&self) -> usize {
        self.cols
    }

    pub fn h(&self) -> usize {
        self.rows
    }
    pub fn kind(&self) -> &MatKind {
        &self.kind
    }

    pub fn show(&self, title: &str) {
        let mut buf: Vec<u32> = self.data.iter().flatten().copied().collect();
        let mut window = Window::new(title, self.w(), self.h(), WindowOptions::default())
            .unwrap_or_else(|e| panic!("{}", e));
        window.set_target_fps(60);

        while window.is_open() && !window.is_key_down(Key::Escape) {
            for _i in buf.iter_mut() {
                // todo
                // *_i = ?
            }

            window.update_with_buffer(&buf, self.w(), self.h()).unwrap();
        }
    }

    pub fn dct(&self) -> Vec<f64> {
        let buf: Vec<f64> = self.data.iter().flatten().map(|x| *x as f64).collect();
        one::dct(&buf)
    }

    pub fn idct(&mut self, dct: Vec<f64>) -> &Mat {
        let idct: Vec<u32> = one::idct(&dct).iter().map(|x| *x as u32).collect();
        self.data = idct
            .chunks(self.cols)
            .map(|c| c.to_vec())
            .collect::<Vec<Vec<u32>>>();
        self
    }
}
