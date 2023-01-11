#![allow(dead_code)]

use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, usize> = HashMap::new();

        for (index, &num) in nums.iter().enumerate() {
            if let Some(i) = map.get(&(target - num)) {
                return vec![index as i32, *i as i32];
            }
            map.insert(num, index);
        }

        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_two_sum() {
        assert_eq!(vec![2, 0], Solution::two_sum(vec![2, 8, 7, 15], 9));
        assert_eq!(vec![2, 1], Solution::two_sum(vec![3, 2, 4], 6));
    }
}
