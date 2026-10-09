/// [1365. How Many Numbers Are Smaller Than the Current Number](https://leetcode.com/problems/how-many-numbers-are-smaller-than-the-current-number/)
/// frequency counting + prefix sums.

impl Solution {
    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        let mut counts = [0; 101];

        // O(n): Count occurrences
        for &num in &nums {
            counts[num as usize] += 1;
        }

        // O(k): Calculate prefix sums
        let mut total = 0;

        for count in &mut counts {
            let current = *count;
            *count = total;
            total += current;
        }

        // O(n): Build result
        return nums.iter().map(|&num| counts[num as usize]).collect();
    }
}
