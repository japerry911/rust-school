fn summation(n: i32) -> i32 {
    let mut total: i32 = 0;

    for i in 1..n + 1 {
        total += i;
    }

    total
}
