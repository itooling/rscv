use rand::Rng;
use rscv::{
    cv::Mat,
    one::{dct, idct},
};

#[test]
fn test_dct() {
    let mut rng = rand::thread_rng();
    let input: Vec<f64> = (0..64).map(|_| rng.gen_range(0.0..255.0)).collect();

    println!("---input: {:?}", input);
    let mut dct_out = dct(&input);

    dct_out[18] += 10f64;

    let idct_out = idct(&dct_out);
    println!("---output: {:?}", idct_out);
}

#[test]
fn test_rgb() {
    let rgb = Mat::from_rgb(10, 100, 200);
    println!("rgb: {}", rgb);
    println!("r g b: {:?}", Mat::to_rgb(rgb));
}
