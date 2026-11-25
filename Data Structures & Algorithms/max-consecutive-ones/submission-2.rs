use std::any::type_name_of_val;

impl Solution {


    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut max: i32 = 0;
        let mut start: usize = 0;
        let mut end: usize = nums.len();
        let mut counter: i32 = 0;

        for i in start..end{

            if nums[i] == 1{
                counter += 1;
            }
            max = max.max(counter);
            if nums[i] == 0{
                counter = 0; 
            }
        }
        max
    }
}
