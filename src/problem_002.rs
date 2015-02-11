fn main() {
    let mut first = 1;
    let mut second = 2;
    let mut total_of_evens_fibs = 2;

    while first + second <= 4000000 {
        let next = first + second;
        if next % 2 == 0 { total_of_evens_fibs += next; }
        first = second; second = next;
    }
    println!("{}", total_of_evens_fibs);
}
