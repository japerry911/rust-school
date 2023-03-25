fn digital_root(n: i64) -> i64 {
    let mut digits_vector: Vec<i64> = Vec::new();
    let mut total: i64 = n;
    let mut tmp_n: i64;

    loop {
        tmp_n = total;

        digits_vector.truncate(0);

        while tmp_n > 0 {
            let sub_n: i64 = tmp_n % 10;

            digits_vector.push(sub_n);

            tmp_n /= 10;
        }

        total = digits_vector.iter().sum();

        if total < 10 {
            break;
        }
    }

    total
}
