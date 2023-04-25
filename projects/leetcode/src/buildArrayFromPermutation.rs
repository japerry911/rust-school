impl Solution {
    pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
        let mut return_vec: Vec<i32> = vec![];

        for num in &nums {
            let use_num = nums[*num as usize];
            return_vec.push(use_num);
        }

        return_vec
    }
}
