use std::any::type_name_of_val;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        if nums.is_empty(){
            return 0;
        }

        let mut left: usize = 0; 
        let mut right: usize = nums.len();

        while left < right {
            if nums[left] == val{
                right -= 1;
                nums.swap(left, right);
            }else{
              left  += 1;
            }    
            
        }
       
        right as i32
    }
}
