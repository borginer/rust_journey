//https://leetcode.com/problems/two-sum/description

use std::collections::HashMap;

fn main() {
    let nums = vec![3, 3];
    let indices = two_sum(nums, 6);
    println!("indices: {indices:?}");
}

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();
    for (i, num) in nums.iter().enumerate() {
        match map.get(num) {
            Some(n) => {
                return vec![i as i32, *n];
            }
            None => {
                map.insert(target - num, i as i32);
            },
        }
    }

    Vec::new()
}
