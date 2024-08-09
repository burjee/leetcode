struct StockSpanner {
    prices: Vec<(usize, i32)>,
}
impl StockSpanner {
    fn new() -> Self {
        StockSpanner { prices: vec![] }
    }

    fn next(&mut self, price: i32) -> i32 {
        let mut span = 1;
        while let Some(&(last_span, last_price)) = self.prices.last() {
            if last_price <= price {
                span += last_span;
                self.prices.pop();
            } else {
                break;
            }
        }
        self.prices.push((span, price));
        span as i32
    }
}

pub fn run() {
    enum Cmd {
        StockSpanner,
        Next(i32),
    }

    let input = [vec![
        Cmd::StockSpanner,
        Cmd::Next(100),
        Cmd::Next(80),
        Cmd::Next(60),
        Cmd::Next(70),
        Cmd::Next(60),
        Cmd::Next(75),
        Cmd::Next(85),
    ]];

    let mut stock_spanner = StockSpanner::new();
    for commands in input {
        for cmd in commands {
            match cmd {
                Cmd::StockSpanner => {
                    stock_spanner = StockSpanner::new();
                    print!("null, ");
                }
                Cmd::Next(price) => print!("{}, ", stock_spanner.next(price)),
            }
        }
    }
}
