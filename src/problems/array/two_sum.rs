use std::collections::HashMap;

use crate::Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();

        for (i, num) in nums.iter().enumerate() {
            let complement = target - num;

            if let Some(&index) = map.get(&complement) {
                return vec![index, i as i32];
            }

            map.insert(num, i as i32);
        }

        vec![]
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_two_sum() {
        let result = Solution::two_sum(vec![4, 5, 2], 6);
        assert_eq!(result, vec![0, 2]);
    }
}
