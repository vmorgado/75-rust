pub mod best_time_to_buy_stock {

    impl crate::solution::leet_code_solutions::Solution {
        pub fn max_profit(prices: Vec<i32>) -> i32 {
            let mut profit = 0;
            let mut min_price = i32::MAX;

            for price in prices.iter() {
                if price < &min_price {
                    min_price = *price;
                } else if price - min_price > profit {
                    profit = price - min_price;
                }
            }

            profit
        }
    }
}
