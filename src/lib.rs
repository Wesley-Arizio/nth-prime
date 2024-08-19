struct Prime {
    current: u32,
}

impl Prime {
    pub fn new() -> Self {
        Self {
            current: 2,
        }
    }

    fn is_prime(n: u32) -> bool {
        if n <= 1 {
            return false;
        }
        if n == 2 || n == 3 {
            return true;
        }
        if n % 2 == 0 || n % 3 == 0 {
            return false;
        }
        let mut i = 5;
        while i * i <= n {
            if n % i == 0 || n % (i + 2) == 0 {
                return false;
            }
            i += 6;
        }
        true
    }
}

impl Iterator for Prime {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        while !Prime::is_prime(self.current) {
            self.current += 1;
        }
        let prime = self.current;
        self.current += 1;
        Some(prime)
    }
}

pub fn nth(n: u32) -> u32 {
    Prime::new().nth(n as usize).unwrap() as u32
}
