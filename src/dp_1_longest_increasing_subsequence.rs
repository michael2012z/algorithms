// Calculate the length of the longest increasing subsequence
// Labuladong book P.96
fn longth_of_lis(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    let mut dp = Vec::new();
    for _i in 0..len {
        dp.push(1);
    }

    for i in 0..len {
        for j in 0..i {
            if nums[i] > nums[j] {
                dp[i] = std::cmp::max(dp[i], dp[j] + 1)
            }
        }
    }
    *dp.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longth_of_lis() {
        assert_eq!(longth_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4)
    }
}
