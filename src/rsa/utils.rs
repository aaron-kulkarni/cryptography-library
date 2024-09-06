pub struct RSAConfig {
    pub encrypt: bool,
    pub generate: bool,
    pub message: Vec<u8>,
}

fn check_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    };
    if n == 2 {
        return true;
    };
    if n % 2 == 0 {
        return false;
    };

    let max_check: u32 = (n as f64).sqrt() as u32;
    for i in (3..max_check + 1).step_by(2) {
        if n % i as u64 == 0 {
            return false;
        }
    }

    return true;
}
