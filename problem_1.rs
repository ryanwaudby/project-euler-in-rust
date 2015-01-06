fn main() {
    let mut total = 0i;

    for i in range(1i, 1000) {
        if i % 3 == 0 || i % 5 == 0 { total += i }
    }
    println!("{}", total);
}
