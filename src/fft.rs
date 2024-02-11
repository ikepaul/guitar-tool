use std::f32::consts::PI;

pub fn fft(samples: Vec<f32>) -> Vec<(f32, f32)> {
    let size = samples.len();
    if size == 1 {
        return vec![(samples[0], 0.0)];
    };

    let (even_samples, odd_samples) = evens_and_odds(samples);

    let even_res = fft(even_samples);
    let odd_res = fft(odd_samples);

    let mut out: Vec<(f32, f32)> = vec![(0.0, 0.0); size];

    for k in 0..(size / 2) {
        let (o_real, o_imaginary) = odd_res[k];
        let (e_real, e_imaginary) = even_res[k];

        let angle: f32 = -2.0 * PI * (k as f32) / (size as f32);

        let t_real = angle.cos() * o_real - angle.sin() * o_imaginary;
        let t_imaginary = angle.sin() * o_real + angle.cos() * o_imaginary;

        out[k] = (e_real + t_real, e_imaginary + t_imaginary);
        out[k + size / 2] = (e_real - t_real, e_imaginary - t_imaginary);
    }
    out
}

fn evens_and_odds<T: Copy>(arr: Vec<T>) -> (Vec<T>, Vec<T>) {
    let mut evens = Vec::new();
    let mut odds = Vec::new();
    for (index, &element) in arr.iter().enumerate() {
        if index % 2 == 0 {
            evens.push(element);
        } else {
            odds.push(element);
        }
    }
    (evens, odds)
}
