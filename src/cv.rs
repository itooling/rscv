use minifb::{Key, Window, WindowOptions};

pub type ColorRgb = (u8, u8, u8);
pub type ColorRgba = (u8, u8, u8, u8);
pub type MatSize = (usize, usize);
pub type MatData = Vec<Vec<u32>>;

#[derive(Default, Clone)]
pub enum MatKind {
    Gray,
    U8C1,
    U8C2,
    #[default]
    U8C3,
    U8C4,
    U32C1,
    U32C2,
    U32C3,
    U32C4,
}

pub enum Color {
    RGB(u8, u8, u8),
    RGBA(u8, u8, u8, u8),
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
    data: Option<MatData>,
}

impl Mat {
    pub fn new() -> Self {
        Mat {
            rows: 0,
            cols: 0,
            chan: 1,
            kind: MatKind::Gray,
            data: None,
        }
    }
    pub fn new_with_size(size: MatSize) -> Self {
        let mut mat = Mat::default();
        mat.cols = size.0;
        mat.rows = size.1;
        mat.chan = 3;
        mat.kind = MatKind::U8C3;
        mat.data = Some(vec![vec![0u32; size.0]; size.1]);
        mat
    }
    pub fn new_with_size_rgb(size: MatSize, rgb: u32) -> Self {
        let mut mat = Mat::default();
        mat.cols = size.0;
        mat.rows = size.1;
        mat.chan = 3;
        mat.kind = MatKind::U8C3;
        mat.data = Some(vec![vec![rgb; size.0]; size.1]);
        mat
    }

    pub fn new_with_size_data(size: MatSize, data: Vec<u32>) -> Self {
        if data.len() != size.0 * size.1 {
            panic!("data size not satisfied");
        }
        let mut mat = Mat::default();
        mat.cols = size.0;
        mat.rows = size.1;
        mat.chan = 3;
        mat.kind = MatKind::U8C3;
        mat.data = Some(
            data.chunks(size.0)
                .map(|x| x.to_vec())
                .collect::<Vec<Vec<u32>>>(),
        );
        mat
    }

    pub fn new_with_data_w(data: Vec<u32>, w: usize) -> Self {
        if data.len() % w != 0 {
            panic!("data size not satisfied");
        }
        let mut mat = Mat::default();
        let h = data.len() / w;
        mat.cols = w;
        mat.rows = h;
        mat.chan = 3;
        mat.kind = MatKind::U8C3;
        mat.data = Some(
            data.chunks(w)
                .map(|x| x.to_vec())
                .collect::<Vec<Vec<u32>>>(),
        );
        mat
    }

    pub fn new_with_data_h(data: Vec<u32>, h: usize) -> Self {
        if data.len() % h != 0 {
            panic!("data size not satisfied");
        }
        let mut mat = Mat::default();
        let w = data.len() / h;
        mat.cols = w;
        mat.rows = h;
        mat.chan = 3;
        mat.kind = MatKind::U8C3;
        mat.data = Some(
            data.chunks(w)
                .map(|x| x.to_vec())
                .collect::<Vec<Vec<u32>>>(),
        );
        mat
    }

    pub fn from_rgb(rgb: ColorRgb) -> u32 {
        let (r, g, b) = (rgb.0 as u32, rgb.1 as u32, rgb.2 as u32);
        (r << 16) | (g << 8) | b
    }

    pub fn to_rgb(rgb: u32) -> ColorRgb {
        (
            (rgb >> 16) as u8 & 0xff,
            (rgb >> 8) as u8 & 0xff,
            rgb as u8 & 0xff,
        )
    }

    pub fn size(&self) -> MatSize {
        (self.cols, self.rows)
    }

    pub fn chan(&self) -> usize {
        self.chan
    }

    pub fn data(&self) -> &Option<MatData> {
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
        if let Some(ref data) = self.data {
            let mut buf: Vec<u32> = data.iter().flatten().copied().collect();
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
    }
}
