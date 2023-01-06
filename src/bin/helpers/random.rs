use rand::Rng;

pub fn random_number(n: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen::<f64>() * n
}

pub fn random_size(n: usize) -> usize {
    random_number(n as f64).round() as usize
}
