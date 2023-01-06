use rand::Rng;

pub fn random_number(n: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen::<f64>() * n
}

pub fn random_size(n: usize) -> usize {
    random_number(n as f64).round() as usize
}

pub fn get_2_random_indices(l: usize) -> (usize, usize){
    let mut x1 = 0;
    let mut x2 = 0;
    while x1 == x2 {
        x1 = random_size(l);
        x2 = random_size(l);
    }
    return (x1, x2);
}
