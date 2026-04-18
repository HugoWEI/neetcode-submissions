impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let mut seen = HashSet::with_capacity(nums.len());

        for n in nums {
            if !seen.insert(n) {
                return true;
            }
        }
        false
    }
}
