impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::with_capacity(nums.len());

        for (i, &n) in nums.iter().enumerate() {
            let difference = target - n;

            if let Some(&j) = map.get(&difference) {
                return vec![j as i32, i as i32];
            }

            map.insert(n, i);
        }

        vec![]
    }
}
