pub mod product_except_self {

    impl crate::solution::leet_code_solutions::Solution {
        pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
            let mut zeros = 0;
            let mut product = 0;
            for num in nums.iter() {
                if *num == 0 {
                    zeros = zeros + 1;
                    continue;
                }

                if product == 0 {
                    product = 1;
                }

                product = product * num;
            }

            if zeros > 1 {
                return nums.into_iter().map(|_| 0).collect();
            }

            nums.into_iter()
                .map(|x| {
                    if zeros == 1 && x == 0 {
                        return product;
                    }
                    if zeros == 1 {
                        return 0;
                    }

                    product / x
                })
                .collect()
        }
    }
}
