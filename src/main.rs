use rscv::cv::Mat;
fn main() {
    Mat::new_with_size_rgb(500, 500, Mat::from_rgb(200, 0, 0)).show("test");
}
