fn narcissistic(num: u64) -> bool {
    let mut tmp_num: u64 = num;
    let mut sub_num: u64;
    let mut num_length: u32 = 0;

    let mut digits_vec = Vec::new();

    while tmp_num > 0 {
        sub_num = tmp_num % 10;

        digits_vec.push(sub_num);
        num_length += 1;

        tmp_num /= 10;
    }

    let exp_sum: u64 = digits_vec.iter().map(|&i| raise(&i, &num_length)).sum();

    num == exp_sum
}

fn raise(i: &u64, power: &u32) -> u64 {
    i.pow(*power)
}
