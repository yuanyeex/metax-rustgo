impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for (num, i) in nums.iter {
            println!("{} {} {}", num, i, target - num);
        }
    }
}

fn main() {
    println!("Hello world");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_two_sum() {
    }
}