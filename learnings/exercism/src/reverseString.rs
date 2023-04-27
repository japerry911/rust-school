pub fn reverse(input: &str) -> String {
    let mut reversed_input = String::from("");

    for c in input.chars().rev() {
        reversed_input.push(c);
    }

    reversed_input
}
