impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut total: i32 = 0;

        let mut previous_i: bool = false;
        let mut previous_x: bool = false;
        let mut previous_c: bool = false;

        for c in s.chars() {
            if c == 'I' {
                previous_i = true;
                total += 1;
            } else if c == 'V' {
                if previous_i == true {
                    total += 3;
                    previous_i = false;
                } else {
                    total += 5;
                }
            } else if c == 'X' {
                if previous_i == true {
                    total += 8;
                    previous_i = false;
                } else {
                    previous_x = true;
                    total += 10;
                }
            } else if c == 'L' {
                if previous_x == true {
                    total += 30;
                    previous_x = false;
                } else {
                    total += 50;
                }
            } else if c == 'C' {
                if previous_x == true {
                    total += 80;
                    previous_x = false;
                } else {
                    previous_c = true;
                    total += 100;
                }
            } else if c == 'D' {
                if previous_c == true {
                    total += 300;
                    previous_c = false;
                } else {
                    total += 500;
                }
            } else if c == 'M' {
                if previous_c == true {
                    total += 800;
                    previous_c = false;
                } else {
                    total += 1000;
                }
            }
        }

        total
    }
}
