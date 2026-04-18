impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<[i32; 26], Vec<String>> = HashMap::new();

        for word in strs {
            let mut key = [0; 26];

            for c in word.bytes() {
                key[(c - b'a') as usize] += 1;
            }

            map.entry(key).or_default().push(word);
        }

        map.into_values().collect()
    }
}
