type Length = usize;
type Price = usize;
type Solution = usize;

pub struct Rod {
    pub length: Length
}

pub fn rod_cutter(rod: Rod, lengths: Vec<Length>, prices: Vec<Price>) -> Solution {
    let mut dp = vec![0; rod.length + 1];
    for l in 1..=rod.length {
        for i in 0..lengths.len() {
            if lengths[i] <= l {
                dp[l] = dp[l].max(prices[i] + dp[l - lengths[i]])
            }
        }
    }

    dp[rod.length]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let rod = Rod {length: 10};
        let lengths = vec![1, 2, 3, 4, 5, 6];
        let prices = vec![1, 5, 9, 12, 12, 15];

        let result = rod_cutter(rod, lengths, prices);
        assert_eq!(result, 30);
    }
}
