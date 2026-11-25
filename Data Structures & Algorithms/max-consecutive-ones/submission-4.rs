use std::any::type_name_of_val;

impl Solution {


    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut max: i32 = 0;
        let mut counter: i32 = 0;

        for i in nums{
            if i == 1{
                counter += 1;
                if counter > max {
                   max = max.max(counter); 
                }  
            }else{
                counter = 0; 
            }
        }
        max
    }
}
