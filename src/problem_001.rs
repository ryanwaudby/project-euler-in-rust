fn main() {
    println!("{}", range(1is, 1000is).filter(|&i| i % 3 == 0 || i % 5 == 0).fold(0, |acc, i| acc + i));
}
