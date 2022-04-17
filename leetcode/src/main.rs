use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut cache:HashMap<i32, i32> = HashMap::new();
        for (index, elem) in nums.iter().enumerate() {
            if let Some(ind) =  cache.get(elem) {
                return vec![*ind, index as i32];
            } else {
                cache.insert(target - elem, index as i32);
            }
        }
        vec![-1, -1]
    }
}


fn main() {
    let ret = Solution::two_sum(vec!(1, 32, 21), 53);
    println!("{:?}", ret);
}
