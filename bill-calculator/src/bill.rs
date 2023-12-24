use std::ops::RangeInclusive;

#[derive(Debug)]
pub struct Bill {
    pub price_range: (RangeInclusive<u32>, f32),
    pub daily_usage: Vec<u32>,
}

impl Bill {
    pub const fn new() -> Self {
        Bill {
            price_range: (1..=4, 2.5),
            daily_usage: Vec::new(),
        }
    }

    pub fn change_daily_usage(&mut self, daily_usage: Vec<u32>) -> &mut Self {
        self.daily_usage = daily_usage;
        self
    }

    pub fn change_price_range(&mut self, price_range: (RangeInclusive<u32>, f32)) -> &mut Self {
        self.price_range = price_range;
        self
    }

    pub fn calculate(&self) -> f32 {
        let mut bill = 0.0;

        for hourly_usage in self.daily_usage.iter() {
            match hourly_usage {
                1..=4 => bill += *hourly_usage as f32 * self.price_range.1,
                _ => bill += *hourly_usage as f32 * 3.5,
            }
        }

        bill
    }
}
