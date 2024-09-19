use image::EncodableLayout;
use minifb::{Key, Window, WindowOptions};

use crate::one;

pub trait DataKind: Clone {
    type Kind;

    /// get zero
    fn zero() -> u32;

    /// set color
    fn color(color: u32, size: MatSize, chan: usize) -> Vec<Self::Kind>;

    fn buffer(data: &Vec<Self>, chan: usize) -> Vec<u32>;
}

pub trait MatKind {
    type Kind;
}

impl DataKind for u8 {
    type Kind = u8;

    fn zero() -> u32 {
        0u32
    }

    fn color(color: u32, size: MatSize, chan: usize) -> Vec<Self> {
        match chan {
            1 => {
                let p = one::to_argb(color);
                vec![p.3; size.0 * size.1]
            }
            2 => {
                let p = one::to_argb(color);
                let pp = vec![vec![p.2, p.3]; size.0 * size.1];
                pp.into_iter().flatten().collect()
            }
            3 => {
                let p = one::to_argb(color);
                let pp = vec![vec![p.1, p.2, p.3]; size.0 * size.1];
                pp.into_iter().flatten().collect()
            }
            4 => {
                let p = one::to_argb(color);
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
    type Kind = u32;

    fn zero() -> u32 {
        0u32
    }

    fn color(color: u32, size: MatSize, chan: usize) -> Vec<Self> {
        match chan {
            1 | 2 | 3 | 4 => {
                vec![color; size.0 * size.1]
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

#[derive(Clone)]
pub enum MatMode {
    Gray,
    U8C1,
    U8C2,
    U8C3,
    U8C4,
    /// use 32-bit pixel.
    /// The encoding for each pixel is 0RGB:
    /// The upper 8-bits are ignored, the next 8-bits are for the red channel,
    /// the next 8-bits afterwards for the green channel, and the lower 8-bits for the blue channel.
    U32S3,
    /// use 32-bit pixel.
    /// The encoding for each pixel is ARGB:
    /// The upper 8-bits are alpha, the next 8-bits are for the red channel,
    /// the next 8-bits afterwards for the green channel, and the lower 8-bits for the blue channel.
    U32S4,
}

impl MatMode {
    /// return chan and data
    pub fn kind<D: DataKind>(&self, size: MatSize) -> (usize, Vec<D::Kind>) {
        match self {
            MatMode::Gray => (1, D::color(D::zero(), size, 1)),
            MatMode::U8C1 => (1, D::color(D::zero(), size, 1)),
            MatMode::U8C2 => (2, D::color(D::zero(), size, 2)),
            MatMode::U8C3 => (3, D::color(D::zero(), size, 3)),
            MatMode::U8C4 => (4, D::color(D::zero(), size, 4)),
            MatMode::U32S3 => (3, D::color(D::zero(), size, 3)),
            MatMode::U32S4 => (4, D::color(D::zero(), size, 4)),
        }
    }

    /// return chan and data
    pub fn kind_color<D: DataKind>(&self, size: MatSize, color: u32) -> (usize, Vec<D::Kind>) {
        match self {
            MatMode::Gray => (1, D::color(color, size, 1)),
            MatMode::U8C1 => (1, D::color(color, size, 1)),
            MatMode::U8C2 => (2, D::color(color, size, 2)),
            MatMode::U8C3 => (3, D::color(color, size, 3)),
            MatMode::U8C4 => (4, D::color(color, size, 4)),
            MatMode::U32S3 => (3, D::color(color, size, 3)),
            MatMode::U32S4 => (4, D::color(color, size, 4)),
        }
    }
}

impl MatKind for MatMode {
    type Kind = u8;
}

pub enum Color {
    RGB(u8, u8, u8),
    RGBA(u8, u8, u8, u8),
}

#[derive(Clone)]
pub struct Mat<T = u8> {
    w: usize,
    h: usize,
    chan: usize,
    kind: MatMode,
    data: MatData<T>,
}

impl Mat<u8> {
    pub fn open(path: &str) -> Mat<u8> {
        let img = image::open(path).expect("open file error");
        let rgb = img.to_rgb8();
        let data = rgb.as_bytes().to_vec();
        Mat {
            w: img.width() as usize,
            h: img.height() as usize,
            chan: 3,
            kind: MatMode::U8C3,
            data: data,
        }
    }
}

impl<D> Mat<D>
where
    D: DataKind,
{
    pub fn new(size: MatSize, kind: MatMode) -> Mat<D::Kind> {
        let (chan, data) = kind.kind::<D>(size);
        Mat {
            w: size.0,
            h: size.1,
            chan: chan,
            kind: kind,
            data: data,
        }
    }

    pub fn new_color(size: MatSize, kind: MatMode, color: u32) -> Mat<D::Kind> {
        let (chan, data) = kind.kind_color::<D>(size, color);
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

    pub fn w(&self) -> usize {
        self.w
    }

    pub fn h(&self) -> usize {
        self.h
    }
    pub fn kind(&self) -> &MatMode {
        &self.kind
    }

    pub fn show(&self, title: &str) {
        let mut buffer = D::buffer(&self.data, self.chan);
        let mut window = Window::new(title, self.w, self.h, WindowOptions::default())
            .unwrap_or_else(|e| panic!("{}", e));
        window.set_target_fps(60);

        while window.is_open() && !window.is_key_down(Key::Escape) {
            for _i in buffer.iter_mut() {
                // todo
                // *_i = ?
            }
            window.update_with_buffer(&buffer, self.w, self.h).unwrap();
        }
    }
}
