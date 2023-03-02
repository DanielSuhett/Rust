pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let length = nums.len();
    let mut result: Vec<i32> = vec![];

    for x in 0..length {
        for y in 0..length {
            if x != y && (nums[x] + nums[y] == target) {
                result.push(nums[x] as i32);
            }
        }
    }

    return result;
}
