pub mod add_two_numbers {

    use crate::utils::leetcode_utils::ListNode;

    fn get_node(node: Option<Box<ListNode>>) -> ListNode {
        match node {
            Some(val) => *val,
            None => ListNode::new(0),
        }
    }

    fn check_next_node_exists(node: &Option<Box<ListNode>>) -> bool {
        match node {
            Some(_val) => true,
            None => false,
        }
    }

    fn calculate_result(
        mut curr_result: Box<ListNode>,
        n1: Option<Box<ListNode>>,
        n2: Option<Box<ListNode>>,
        remainder: i32,
    ) -> Option<Box<ListNode>> {
        let node_one = get_node(n1);
        let node_two = get_node(n2);

        let result_ref = curr_result.as_mut();
        let val = (node_one.val + node_two.val + remainder) % 10;
        let rem: i32 = (node_one.val + node_two.val + remainder) / 10;

        result_ref.val = val;

        if check_next_node_exists(&node_one.next)
            || check_next_node_exists(&node_two.next)
            || rem > 0
        {
            let next_result_node = Box::new(ListNode::new(0));
            result_ref.next = calculate_result(next_result_node, node_one.next, node_two.next, rem);
        };

        Some(curr_result)
    }

    impl crate::solution::leet_code_solutions::Solution {
        pub fn add_two_numbers(
            l1: Option<Box<ListNode>>,
            l2: Option<Box<ListNode>>,
        ) -> Option<Box<ListNode>> {
            let result = match calculate_result(Box::new(ListNode::new(0)), l1, l2, 0) {
                Some(l) => l,
                None => Box::new(ListNode::new(0)),
            };

            Some(result)
        }
    }
}
