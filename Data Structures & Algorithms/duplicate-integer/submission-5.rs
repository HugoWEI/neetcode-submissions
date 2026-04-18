impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let mut count: u8 = 0;
        let length= nums.len();
        for i in 0..length{
            for x in i+1..length {
                if nums[i] == nums[x] {
                    count +=1;
                }
            }
        }
        if count>0 {
            true
        } else {
            false
        }
    }
}
