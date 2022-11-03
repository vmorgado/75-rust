pub mod two_sums {

    use std::collections::HashMap;

    impl crate::solution::leet_code_solutions::Solution {
        pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
            let mut diff_hash = HashMap::<i32, i32>::new();
            for (i, num) in nums.into_iter().enumerate() {
                let diff: i32 = target - num;
                if diff_hash.contains_key(&num) {
                    let diff_index = match diff_hash.get(&num) {
                        Some(value) => *value,
                        None => 0,
                    };
                    return Vec::from([diff_index, *&i as i32]);
                }
                diff_hash.insert(diff, *&i as i32);
            }

            Vec::from([0, 0])
        }
    }
}
