mod solution;
mod utils;
use crate::solution::leet_code_solutions::Solution;
fn main() {
    println!(
        "Result: {}",
        Solution::max_sub_array(Vec::from([-2, 1, -3, 4, -1, 2, 1, -5, 4])),
    )
}

#[cfg(test)]
mod tests {

    use crate::solution::leet_code_solutions::Solution;
    use crate::utils::leetcode_utils::ListNode;

    #[test]
    fn max_sub_array() {
        assert_eq!(
            Solution::max_sub_array(Vec::from([-2, 1, -3, 4, -1, 2, 1, -5, 4])),
            6
        );
        assert_eq!(Solution::max_sub_array(Vec::from([1])), 1);
        assert_eq!(Solution::max_sub_array(Vec::from([5, 4, -1, 7, 8])), 23);
    }

    #[test]
    fn product_except_self() {
        assert_eq!(
            Solution::product_except_self(Vec::from([1, 2, 3, 4])),
            Vec::from([24, 12, 8, 6])
        );
        assert_eq!(
            Solution::product_except_self(Vec::from([-1, 1, 0, -3, 3])),
            Vec::from([0, 0, 9, 0, 0])
        );
    }

    #[test]
    fn contains_duplicate() {
        assert_eq!(Solution::contains_duplicate(Vec::from([1, 2, 3, 1])), true);
        assert_eq!(Solution::contains_duplicate(Vec::from([1, 2, 3, 4])), false);
        assert_eq!(
            Solution::contains_duplicate(Vec::from([1, 1, 1, 3, 3, 4, 3, 2, 4, 2])),
            true
        );
    }

    #[test]
    fn max_profit() {
        assert_eq!(Solution::max_profit(Vec::from([7, 1, 5, 3, 6, 4])), 5);
        assert_eq!(Solution::max_profit(Vec::from([7, 6, 4, 3, 1])), 0);
    }

    #[test]
    fn two_sum() {
        let result = Solution::two_sum(Vec::from([0, 1, 2, 3]), 3);
        assert_eq!(result, [1, 2]);
    }

    #[test]
    fn lenght_of_longest_substring() {
        let result = Solution::length_of_longest_substring(String::from("abcanmm"));
        assert_eq!(result, 5);
    }

    #[test]
    fn add_two_number() {
        let list_one = ListNode {
            val: 9,
            next: Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode { val: 9, next: None })),
            })),
        };

        let list_two = ListNode {
            val: 9,
            next: Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode {
                    val: 9,
                    next: Some(Box::new(ListNode { val: 9, next: None })),
                })),
            })),
        };

        let expected = ListNode {
            val: 8,
            next: Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode {
                    val: 9,
                    next: Some(Box::new(ListNode {
                        val: 0,
                        next: Some(Box::new(ListNode { val: 1, next: None })),
                    })),
                })),
            })),
        };

        let result = Solution::add_two_numbers(Some(Box::new(list_one)), Some(Box::new(list_two)));
        assert_eq!(result, Some(Box::new(expected)));
    }
}
