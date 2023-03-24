impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut current_1_index: usize = 0;
        let mut current_2_index: usize = 0;
        let mut current_sum = 0;

        let nums_len: usize = nums.len();

        while current_1_index < nums_len {
            current_sum = nums[current_1_index];
            current_2_index = 0;

            while current_2_index < nums_len {
                if current_2_index != current_1_index {
                    current_sum += nums[current_2_index];

                    if current_sum == target {
                        return vec![current_1_index as i32, current_2_index as i32];
                    } else {
                        current_sum -= nums[current_2_index];
                    }
                }

                current_2_index += 1;
            }

            current_1_index += 1;
        }

        vec![current_1_index as i32, current_2_index as i32]
    }
}
