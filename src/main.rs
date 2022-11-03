mod solution;
mod utils;

fn main() {
    use solution::leet_code_solutions::Solution;
    use utils::leetcode_utils::ListNode;

    println!(
        "Two Sums result: {:?}",
        Solution::two_sum(Vec::from([0, 1, 2, 3]), 3)
    );

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

    println!(
        "Add Two Numbers result: {:?}",
        Solution::add_two_numbers(Some(Box::new(list_one)), Some(Box::new(list_two)))
    );

    println!(
        "Long Substring Without Repeating Chars: {:?}",
        Solution::length_of_longest_substring(String::from("abcanmm"))
    );
}
