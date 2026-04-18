impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut count = HashMap::new();
        
        for i in nums{
            *count.entry(i).or_insert(0) += 1;
        }

        let mut pairs: Vec<(i32, i32)> = count.into_iter().collect();
        pairs.sort_by(|a, b| b.1.cmp(&a.1));

        pairs.iter().take(k as usize).map(|&(n, _)| n).collect()
    }
}

