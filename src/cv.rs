use minifb::{Key, Window, WindowOptions};

use crate::one;

pub trait DataKind: Clone {
    fn zero() -> Self;
    fn pix(pix: Self, size: MatSize, chan: usize) -> Vec<Self>;
    fn argb(argb: u32, size: MatSize, chan: usize) -> Vec<Self>;
    fn buffer(data: &Vec<Self>, chan: usize) -> Vec<u32>;
}

impl DataKind for u8 {
    fn zero() -> Self {
        0u8
    }

    fn pix(pix: Self, size: MatSize, chan: usize) -> Vec<Self> {
        vec![pix; size.0 * size.1 * chan]
    }

    fn argb(argb: u32, size: MatSize, chan: usize) -> Vec<Self> {
        match chan {
            1 => {
                let p = one::to_argb(argb);
                vec![p.3; size.0 * size.1]
            }
            2 => {
                let p = one::to_argb(argb);
                let pp = vec![vec![p.2, p.3]; size.0 * size.1];
                pp.into_iter().flatten().collect()
            }
            3 => {
                let p = one::to_argb(argb);
                let pp = vec![vec![p.1, p.2, p.3]; size.0 * size.1];
                pp.into_iter().flatten().collect()
            }
            4 => {
                let p = one::to_argb(argb);
                let pp = vec![vec![p.0, p.1, p.2, p.3]; size.0 * size.1];
                pp.into_iter().flatten().collect()
            }
            _ => {
                vec![]
            }
        }
    }

    fn buffer(data: &Vec<Self>, chan: usize) -> Vec<u32> {
        match chan {
            1 => data
                .chunks(chan)
                .map(|x| one::from_argb(0, 0, 0, x[0]))
                .collect(),
            2 => data
                .chunks(chan)
                .map(|x| one::from_argb(0, 0, x[0], x[1]))
                .collect(),
            3 => data
                .chunks(chan)
                .map(|x| one::from_argb(0, x[0], x[1], x[2]))
                .collect(),
            4 => data
                .chunks(chan)
                .map(|x| one::from_argb(x[0], x[1], x[2], x[3]))
                .collect(),
            _ => {
                vec![]
            }
        }
    }
}

impl DataKind for u32 {
    fn zero() -> Self {
        0u32
    }

    fn pix(pix: Self, size: MatSize, chan: usize) -> Vec<Self> {
        match chan {
            1 | 2 | 3 | 4 => {
                vec![pix; size.0 * size.1]
            }
            _ => {
                vec![]
            }
        }
    }

    fn argb(argb: u32, size: MatSize, chan: usize) -> Vec<Self> {
        match chan {
            1 | 2 | 3 | 4 => {
                vec![argb; size.0 * size.1]
            }
            _ => {
                vec![]
            }
        }
    }

    fn buffer(data: &Vec<Self>, chan: usize) -> Vec<u32> {
        match chan {
            1 | 2 | 3 | 4 => data.clone(),
            _ => {
                vec![]
            }
        }
    }
}

pub type ColorRgb = (u8, u8, u8);
pub type ColorArgb = (u8, u8, u8, u8);
pub type MatSize = (usize, usize);
pub type MatData<T> = Vec<T>;

pub enum MatKind {
    Gray,
    U8C1,
    U8C2,
    U8C3,
    U8C4,
    /// use 32-bit pixel.
    /// The encoding for each pixel is 0RGB:
    /// The upper 8-bits are ignored, the next 8-bits are for the red channel,
    /// the next 8-bits afterwards for the green channel, and the lower 8-bits for the blue channel.
    U32B3,
    /// use 32-bit pixel.
    /// The encoding for each pixel is ARGB:
    /// The upper 8-bits are alpha, the next 8-bits are for the red channel,
    /// the next 8-bits afterwards for the green channel, and the lower 8-bits for the blue channel.
    U32B4,
}

impl MatKind {
    /// return chan and data
    pub fn kind<T: DataKind>(&self, size: MatSize) -> (usize, Vec<T>) {
        match self {
            MatKind::Gray => (1, T::pix(T::zero(), size, 1)),
            MatKind::U8C1 => (1, T::pix(T::zero(), size, 1)),
            MatKind::U8C2 => (2, T::pix(T::zero(), size, 2)),
            MatKind::U8C3 => (3, T::pix(T::zero(), size, 3)),
            MatKind::U8C4 => (4, T::pix(T::zero(), size, 4)),
            MatKind::U32B3 => (3, T::pix(T::zero(), size, 3)),
            MatKind::U32B4 => (4, T::pix(T::zero(), size, 4)),
        }
    }

    /// return chan and data
    pub fn kind_with_pix<T: DataKind>(&self, size: MatSize, pix: u32) -> (usize, Vec<T>) {
        // let
        match self {
            MatKind::Gray => (1, T::argb(pix, size, 1)),
            MatKind::U8C1 => (1, T::argb(pix, size, 1)),
            MatKind::U8C2 => (2, T::argb(pix, size, 2)),
            MatKind::U8C3 => (3, T::argb(pix, size, 3)),
            MatKind::U8C4 => (4, T::argb(pix, size, 4)),
            MatKind::U32B3 => (3, T::argb(pix, size, 3)),
            MatKind::U32B4 => (4, T::argb(pix, size, 4)),
        }
    }
}

pub enum Color {
    RGB(u8, u8, u8),
    RGBA(u8, u8, u8, u8),
}

pub struct Mat<T: DataKind> {
    w: usize,
    h: usize,
    chan: usize,
    kind: MatKind,
    data: MatData<T>,
}

impl<T: DataKind> Mat<T> {
    pub fn new(size: MatSize, kind: MatKind) -> Mat<T> {
        let (chan, data) = kind.kind(size);

        Mat {
            w: size.0,
            h: size.1,
            chan: chan,
            kind: kind,
            data: data,
        }
    }

    pub fn new_with_size_pix(size: MatSize, kind: MatKind, pix: u32) -> Mat<T> {
        let (chan, data) = kind.kind_with_pix(size, pix);

        Mat {
            w: size.0,
            h: size.1,
            chan: chan,
            kind: kind,
            data: data,
        }
    }

    pub fn size(&self) -> MatSize {
        (self.w, self.h)
    }

    pub fn chan(&self) -> usize {
        self.chan
    }

    pub fn data(&self) -> &MatData<T> {
        &self.data
    }

    pub fn w(&self) -> usize {
        self.w
    }

    pub fn h(&self) -> usize {
        self.h
    }
    pub fn kind(&self) -> &MatKind {
        &self.kind
    }

    pub fn show(&self, title: &str) {
        let mut buffer = T::buffer(&self.data, self.chan);
        let mut window = Window::new(title, self.w(), self.h(), WindowOptions::default())
            .unwrap_or_else(|e| panic!("{}", e));
        window.set_target_fps(60);

        while window.is_open() && !window.is_key_down(Key::Escape) {
            for _i in buffer.iter_mut() {
                // todo
                // *_i = ?
            }
            window
                .update_with_buffer(&buffer, self.w(), self.h())
                .unwrap();
        }
    }
}
