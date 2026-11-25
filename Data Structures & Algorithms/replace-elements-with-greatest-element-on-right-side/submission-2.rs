impl Solution {
    pub fn replace_elements(arr: Vec<i32>) -> Vec<i32> {
        let mut result = arr;
        let mut max = -1;
        for i in (0..result.len()).rev() {
            let current = result[i];

            result[i] = max;

            max = max.max(current)
        }

        result
    }
}
