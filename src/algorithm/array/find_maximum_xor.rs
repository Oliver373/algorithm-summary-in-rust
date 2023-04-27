
///给你一个整数数组 nums ，返回 nums[i] XOR nums[j] 的最大运算结果，其中 0 ≤ i ≤ j < n 。
///
///来源：力扣（LeetCode）
///链接：https://leetcode.cn/problems/maximum-xor-of-two-numbers-in-an-array

#[derive(Default)]
pub struct Trie {
    pub left: Option<Box<Trie>>,
    pub right: Option<Box<Trie>>,
}

impl Trie {
    fn new() -> Self {
        Default::default()
    }

    pub fn insert(&mut self, num: i32) {
        (0..31).rev().fold(self, |node, k| {
            match (num >> k)&1 {
                0 => node.left.get_or_insert_with(Default::default),
                1 => node.right.get_or_insert_with(Default::default),
                _ => panic!("error input"),
            }
        });
    }

    pub fn find_other(&self, num: i32) -> i32{
        let mut temp = self;
        let mut result:i32 = 0;
        for k in (0..31).rev() {
            match (num>>k)&1 {
                1 => match &temp.left {
                    Some(e) => {
                        temp = &e;
                        result = result<<1;
                    },
                    None => {
                        match &temp.right {
                            Some(e) => {
                                temp = &e;
                                result = (result<<1) + 1;
                            },
                            None => {
                                panic!("error tire tree");
                            }
                        }
                    }
                },
                0 => match &temp.right {
                    Some(e) => {
                        temp = &e;
                        result = (result<<1) + 1;
                    },
                    None => {
                        match &temp.left {
                            Some(e) => {
                                temp = &e;
                                result = result<<1;
                            },
                            None => {
                                panic!("error tire tree");
                            }
                        }
                    }
                },
                _ => panic!("error input"),
            }
        }
        result
    }
}

pub fn find_maximum_xor(nums: Vec<i32>) -> i32 {
    let mut tire = Trie::new();
    let mut max = 0;
    for i in 1..nums.len() {
        tire.insert(nums[i-1]);
        // println!("i: {},num: {}, find_other: {}",i, nums[i], tire.find_other(nums[i]));
        max = max.max(tire.find_other(nums[i])^nums[i]);
    }
    return max;
}

pub fn find_maximum_xor_sample(nums: Vec<i32>) -> i32 {
    let mut max = 0;
    for i in 0..nums.len() {
        for j in i+1..nums.len() {
            max = max.max(nums[i] ^ nums[j]);
        }
    }
    return max;
}

#[cfg(test)]
mod tests{

    use super::*;

    #[test]
    fn test_find_maximum_xor() {
        let nums = vec![3,10,5,25,2,8];
        let target = 28;
        let result = find_maximum_xor(nums.clone());
        assert_eq!(result, target, "\n input:{:?}", nums);
    }

    #[test]
    fn test() {
        let nums = vec![1,2];
        let target = 3;
        let result = find_maximum_xor(nums.clone());
        assert_eq!(result, target, "\n input:{:?}", nums);
    }

}