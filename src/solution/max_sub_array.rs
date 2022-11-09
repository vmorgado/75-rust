pub mod max_sub_array {

    impl crate::solution::leet_code_solutions::Solution {
        pub fn max_sub_array(nums: Vec<i32>) -> i32 {
            let total: i32 = nums.iter().sum();
            let mut max = total;
            let mut iter = 0;
            let mut iter_inv = nums.len() - 1;
            let mut total_r = total;
            let mut total_l = total;
            let mut total_c = total;

            for num in nums.iter() {
                let r_num = match nums.get(iter_inv) {
                    Some(n) => *n,
                    None => 0,
                };

                total_r = total_r - r_num;
                total_l = total_l - num;
                total_c = total_c - r_num - num;

                if total_r > max {
                    max = total_r;
                }

                if total_l > max {
                    max = total_l;
                }

                if total_c > max {
                    max = total_c;
                }

                iter = iter + 1;
                if iter_inv > 0 {
                    iter_inv = iter_inv - 1;
                }
            }

            max
        }
    }
}
