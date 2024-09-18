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
