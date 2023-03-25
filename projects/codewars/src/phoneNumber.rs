fn create_phone_number(numbers: &[u8]) -> String {
    let mut fmt_number = String::new();

    for (i, number) in numbers.iter().enumerate() {
        if i == 0 {
            fmt_number.push_str("(");
        } else if i == 6 {
            fmt_number.push_str("-");
        }

        fmt_number.push_str(&number.to_string());

        if i == 2 {
            fmt_number.push_str(") ");
        }
    }

    fmt_number
}
