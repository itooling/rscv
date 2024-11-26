use image::{DynamicImage, GenericImageView, ImageBuffer, Luma, Pixel};

pub fn open(path: &str) -> DynamicImage {
    image::open(path).expect("open image error")
}

pub fn save(img: DynamicImage, path: &str) {
    img.save(path).expect("save image error");
}

pub fn threshold(img: DynamicImage, threshold: u8) -> ImageBuffer<Luma<i32>, Vec<i32>> {
    let (w, h) = img.dimensions();
    let mut out = ImageBuffer::new(w, h);
    for (x, y, pix) in img.pixels() {
        let luma = pix.to_luma()[0];
        if luma < threshold {
            out.put_pixel(x, y, Luma([0]));
        } else {
            out.put_pixel(x, y, Luma([255]));
        }
    }
    out
}
