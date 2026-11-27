impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut l: usize = m as usize + n as usize;
        let mut first: usize = m as usize;
        let mut second: usize = n as usize;

        while first != 0 && second != 0 {
            if nums1[first-1] > nums2[second-1]{
                nums1[l-1] = nums1[first-1];
                first -= 1;
            }else{
                nums1[l-1] = nums2[second-1];
                second -= 1;
            }
            l -= 1;
        }

        while first != 0 {
            nums1[l-1] = nums1[first-1];
            first -= 1;
            l -= 1;
        }

        while second != 0 {
            nums1[l-1] = nums2[second-1];
            second -= 1;
            l -= 1;
        }
    }
}
