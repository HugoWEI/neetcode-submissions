impl Solution {
    pub fn encode(strs: Vec<String>) -> String {
        let mut res = String::new();

        for word in strs {
            res.push_str(&word.len().to_string());
            res.push('#');
            res.push_str(&word);
        }

        res
    }

    pub fn decode(s: String) -> Vec<String> {
        let mut res = Vec::new();
        let bytes = s.as_bytes();
        let mut i = 0;

        while i < s.len() {
            let mut j = i;
            while bytes[j] != b'#' {
                j += 1;
            }

            let len: usize = s[i..j].parse().unwrap();

            let start = j + 1;
            let end = start + len;

            res.push(s[start..end].to_string());

            i = end;
        }

        res
    }
}
