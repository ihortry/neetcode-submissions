impl Solution {
    pub fn replace_elements(arr: Vec<i32>) -> Vec<i32> {
        if arr.is_empty(){
            return arr;
        }
        let mut result = arr;
        let mut left: usize = 0;
        let right: usize = result.len()-1;

        while left < right {
            let mut max = i32::MIN;
            for &el in &result[left+1..right+1]{
                if el > max{
                    max = el;
                }
            }
            result[left] = max;
            left += 1;
        }
        result[right] = -1;
        result
    }
}
