impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut left = 0;
        let mut right = s.len()-1;
        let mut chars: Vec<char> = s.chars().collect();
        while left < right {
            if !chars[left].is_alphanumeric() {
                left += 1;
                continue;
            }
            if !chars[right].is_alphanumeric() {
                right -= 1;
                continue;
            }

            let a = chars[left].to_ascii_lowercase();
            let b = chars[right].to_ascii_lowercase();
            if a != b {
                return false;
            }
            left += 1;
            right -=1;
        }

        true
    }
}
