pub fn reverse(input: &str) -> String {
    let mut input = String::from(input);
    let mut output = String::from("");

    while let Some(char) = input.pop() {
        output.push(char)
    }

    output
}
