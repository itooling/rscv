use std::f64::consts::PI;

/// dct algorithm
pub fn dct(input: &[f64]) -> Vec<f64> {
    let n = input.len();
    let mut output = vec![0.0; n];

    for k in 0..n {
        let alpha_k = if k == 0 {
            (1.0 / n as f64).sqrt()
        } else {
            (2.0 / n as f64).sqrt()
        };

        let mut sum = 0.0;
        for (i, &x_i) in input.iter().enumerate() {
            sum += x_i * ((PI / n as f64) * (i as f64 + 0.5) * k as f64).cos();
        }
        output[k] = alpha_k * sum;
    }

    output
}

/// idct algorithm
pub fn idct(input: &[f64]) -> Vec<f64> {
    let n = input.len();
    let mut output = vec![0.0; n];

    for i in 0..n {
        let mut sum = 0.0;
        for k in 0..n {
            let alpha_k = if k == 0 {
                (1.0 / n as f64).sqrt()
            } else {
                (2.0 / n as f64).sqrt()
            };
            sum += alpha_k * input[k] * ((PI / n as f64) * (i as f64 + 0.5) * k as f64).cos();
        }
        output[i] = sum;
    }

    output
}

/// use 32-bit pixel.
/// The encoding for each pixel is 0RGB:
/// The upper 8-bits are ignored, the next 8-bits are for the red channel,
/// the next 8-bits afterwards for the green channel, and the lower 8-bits for the blue channel.
pub fn from_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

/// use 32-bit pixel.
/// The encoding for each pixel is 0RGB:
/// The upper 8-bits are ignored, the next 8-bits are for the red channel,
/// the next 8-bits afterwards for the green channel, and the lower 8-bits for the blue channel.
pub fn to_rgb(rgb: u32) -> (u8, u8, u8) {
    (
        0xff & (rgb >> 16) as u8,
        0xff & (rgb >> 8) as u8,
        0xff & (rgb) as u8,
    )
}

/// use 32-bit pixel.
/// The encoding for each pixel is ARGB:
/// The upper 8-bits are alpha, the next 8-bits are for the red channel,
/// the next 8-bits afterwards for the green channel, and the lower 8-bits for the blue channel.
pub fn from_argb(a: u8, r: u8, g: u8, b: u8) -> u32 {
    let (a, r, g, b) = (a as u32, r as u32, g as u32, b as u32);
    (a << 24) | (r << 16) | (g << 8) | b
}

/// use 32-bit pixel.
/// The encoding for each pixel is ARGB:
/// The upper 8-bits are alpha, the next 8-bits are for the red channel,
/// the next 8-bits afterwards for the green channel, and the lower 8-bits for the blue channel.
pub fn to_argb(argb: u32) -> (u8, u8, u8, u8) {
    (
        0xff & (argb >> 24) as u8,
        0xff & (argb >> 16) as u8,
        0xff & (argb >> 8) as u8,
        0xff & (argb as u8),
    )
}

pub fn to_gray(argb: u32) -> u8 {
    0xff & argb as u8
}
