pub mod long_substring_without_repeating_chars {

    use std::collections::HashMap;
    impl crate::solution::leet_code_solutions::Solution {
        pub fn length_of_longest_substring(s: String) -> i32 {
            if s.len() == 0 {
                return 0;
            }
            let s_chars: Vec<char> = s.chars().collect();
            let mut letter_hash = HashMap::<char, i32>::new();

            let mut i = 0;
            let mut char_index = 0;
            let mut ans = 0;
            for c in s_chars {
                if letter_hash.contains_key(&c) {
                    let val = match letter_hash.get(&c) {
                        Some(v) => *v,
                        None => 0,
                    };
                    i = std::cmp::max(val, i);
                    char_index = char_index + 1;
                    ans = std::cmp::max(ans, char_index - i);
                    if let Some(x) = letter_hash.get_mut(&c) {
                        *x = char_index
                    };
                    continue;
                }

                char_index = char_index + 1;
                ans = std::cmp::max(ans, char_index - i);
                letter_hash.insert(c, char_index);
            }

            ans
        }
    }
}
