impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut freq = HashMap::with_capacity(nums.len());

        for &n in &nums {
            *freq.entry(n).or_insert(0) += 1;
        }

        let mut buckets: Vec<Vec<i32>> = vec![Vec::new(); nums.len() + 1];

        for (num, count) in freq {
            buckets[count as usize].push(num);
        }

        let mut res = Vec::with_capacity(k as usize);

        for i in (1..buckets.len()).rev() {
            for &num in &buckets[i] {
                res.push(num);
                if res.len() == k as usize {
                    return res;
                }
            }
        }

        res
    }
}
