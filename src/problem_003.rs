fn main() {
    println!("{}", 3.is_prime());
}

trait PrimeFactorial {
    fn is_prime(&self) -> bool;
}

impl PrimeFactorial for isize {
    fn is_prime(&self) -> bool {
        if *self % 2 == 0 { return false; }
        let mut x = 3;

        loop {
            if x * x <= *self {
                x += 2;
                if *self % x == 0 { return false; }
            } else {
                break;
            }
        }
        return true;
    }
}
