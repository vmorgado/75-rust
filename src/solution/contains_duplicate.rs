    pub mod contains_duplicate {

        use std::collections::HashMap;

        impl crate::solution::leet_code_solutions::Solution {
            pub fn contains_duplicate(nums: Vec<i32>) -> bool {
                let mut hash = HashMap::<i32, bool>::new();

                for num in nums {
                    if hash.contains_key(&num) {
                        return true;
                    }
                    hash.insert(num, true);
                }

                false
            }
        }
    }
