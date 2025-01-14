fn konvolusi(input: &[f32], kernel: &[f32]) -> Vec<f32> {
    let input_len = input.len();
    let kernel_len = kernel.len();
    let mut hasil = Vec::new();
    if kernel_len > input_len {
        panic!("Jumlah Kernel harus lebih kecil atau sama dengan input");
    }
    for i in 0..=(input_len - kernel_len) {
        let mut sum = 0.0;
        for j in 0..kernel_len {
        sum += input[i + j] * kernel[j];
        }
        hasil.push(sum);
    }
    hasil
}
fn main() {
    let input = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let kernel = vec![1.2, 0.5, 0.2];
    let hasill = konvolusi(&input, &kernel);
    println!("Input: {:?}", input);     
    println!("Kernel: {:?}", kernel);
    println!("Hasil Konvolusi: {:?}", hasill);
}
