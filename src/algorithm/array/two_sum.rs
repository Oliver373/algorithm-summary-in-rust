
///给定一个整数数组 nums 和一个整数目标值 target，请你在该数组中找出 和为目标值 target  的那 两个 整数，并返回它们的数组下标。
///
///你可以假设每种输入只会对应一个答案。但是，数组中同一个元素在答案里不能重复出现
///
///来源：力扣（LeetCode）
///链接：https://leetcode.cn/problems/two-sum

use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map:HashMap<i32, usize> = HashMap::new();
    for (index, &num) in nums.iter().enumerate() {
        match map.get(&(target - num)) {
           Some(&v) => return vec![index as i32, v as i32],
           None => map.insert(num, index),
        };
    };
    panic!()
}

pub fn two_sum_sample(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..nums.len() {
        for j in i+1..nums.len() {
            if nums[i]+nums[j] == target { 
                return vec![i as i32,j as i32];
            }
        }
    }
    panic!();
}

#[cfg(test)]
pub mod tests {

    use super::*;

    #[test]
    pub fn test_two_sum() {
        let nums = vec![3,3];
        let target = 6;
        let result = two_sum(nums.clone(), target);
        assert!(result==vec![0,1] || result == vec![1,0],"\n input:{:?}", nums);
    }
}
