fn main() {
    let mut i = 1;
    let mut j = 2;
    let mut total = 0;

    while i + j <= 4000000 {
        let next = i + j;

        if next % 2 == 0 {
            total += i + j;
        }

        i = j;
        j = next;
    }

    println!("{}", total);
}
