impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let bytes = s.as_bytes();       // no copy, just a view
        let mut left = 0;
        let mut right = bytes.len() - 1;

        while left < right {
            if !bytes[left].is_ascii_alphanumeric() {
                left += 1;
                continue;
            }
            if !bytes[right].is_ascii_alphanumeric() {
                right -= 1;
                continue;
            }

            let a = bytes[left].to_ascii_lowercase();
            let b = bytes[right].to_ascii_lowercase();
            if a != b {
                return false;
            }

            left += 1;
            right -= 1;
        }

        true
    }
}