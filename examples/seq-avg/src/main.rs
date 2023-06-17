fn main() {
    seq_f32();

    seq_f64();
}

fn seq_f32() {
    let mut seq: Vec<f32> = vec![1.0, 2.0, 3.0];
    let mut n = 0;
    while seq[0] != seq[1] {
        n += 1;
        let f = avg_f32(&seq);
        seq.insert(1, f);
    }

    println!("Seq: {:?}", seq);
    println!("Iterations: {} (Look at the first two positions in Seq. They are equal.", n);
}

fn avg_f32(seq: &Vec<f32>) -> f32 {
    (seq[1] + seq[0]) / 2.0
}

fn seq_f64() {
    let mut seq: Vec<f64> = vec![1.0, 2.0, 3.0];
    let mut n = 0;
    while seq[0] != seq[1] {
        n += 1;
        let f = avg_f64(&seq);
        seq.insert(1, f);
    }

    println!("Seq: {:?}", seq);
    println!("Iterations: {} (Look at the first two positions in Seq. They are equal.", n);
}

fn avg_f64(seq: &Vec<f64>) -> f64 {
    (seq[1] + seq[0]) / 2.0
}
