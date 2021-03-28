/*
 * @lc app=leetcode.cn id=26 lang=rust
 *
 * [26] 删除排序数组中的重复项
 */

// @lc code=start
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if (nums.len() == 0) {
            return 0;
        }

        let mut index: usize = 1;
        let mut loc: usize =  0;
        while index < nums.len() {
            if (nums[index] == nums[loc]) {
                index += 1;
            } else {
                loc += 1;
                nums[loc] = nums[index];
                index += 1;
            }
        }

        return (loc+1) as i32;
    }
}
// @lc code=end


/*
Accepted
161/161 cases passed (0 ms)
Your runtime beats 100 % of rust submissions
Your memory usage beats 13.04 % of rust submissions (2.3 MB)
*/
