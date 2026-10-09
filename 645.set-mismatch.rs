impl Solution {
    pub fn find_error_nums(mut nums: Vec<i32>) -> Vec<i32> {
        let mut duplicate = 0;

        for i in 0..nums.len() {
            let value = nums[i].abs();
            let index = (value - 1) as usize;

            if nums[index] < 0 {
                duplicate = value;
            } else {
                nums[index] = -nums[index];
            }
        }

        let missing = nums.iter().position(|&value| value > 0).unwrap() as i32 + 1;

        vec![duplicate, missing]
    }
}
