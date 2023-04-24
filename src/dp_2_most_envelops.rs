// Calculate the maximum number of envelops that embed together
// Similar to the longest increasing subsequence puzzle
// Labuladong book P.104
fn max_embedded_envelops(envelops: Vec<(i32, i32)>) -> i32 {
    let mut env = envelops.clone();
    env.sort_by(|a, b| {
        if a.0 == b.0 {
            a.0.cmp(&b.0)
        } else {
            a.1.cmp(&b.1)
        }
    });

    let len = env.len();
    let mut dp = vec![];
    for _ in 0..len {
        dp.push(1)
    }

    for i in 0..len {
        for j in 0..i {
            if env[i].0 > env[j].0 && env[i].1 > env[j].1 {
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
    fn test_max_embedded_envelops() {
        assert_eq!(
            max_embedded_envelops(vec![(5, 4), (6, 4), (6, 7), (2, 3)]),
            3
        )
    }
}
