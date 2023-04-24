// Calculate the maximum summary of sub array
// Labuladong book P.108
fn max_subarray_sum(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    let mut dp = vec![nums[0]];

    for i in 1..len {
        if *dp.last().unwrap() <= 0 {
            dp.push(nums[i])
        } else {
            dp.push(*dp.last().unwrap() + nums[i])
        }
    }
    *dp.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_subarray_sum() {
        assert_eq!(max_subarray_sum(vec![-3, 1, 3, -1, 2, -4, 2]), 5)
    }
}
