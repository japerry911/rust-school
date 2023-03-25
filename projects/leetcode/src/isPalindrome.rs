impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let x_str = x.to_string();

        let x_len: usize = x_str.len();

        let switch_point: usize = x_len / 2;
        let is_even: bool = x_len % 2 == 0;

        let mut left_str = String::from("");
        let mut right_str = String::from("");

        for (i, c) in x_str.chars().enumerate() {
            if i < switch_point {
                left_str.push(c);
            } else if i >= switch_point && is_even == true {
                right_str.push(c);
            } else if i > switch_point {
                right_str.push(c);
            }
        }

        if left_str == right_str.chars().rev().collect::<String>() {
            return true;
        }

        false
    }
}
