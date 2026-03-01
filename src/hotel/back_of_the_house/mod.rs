pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
}

impl Breakfast {
    pub fn summer(item: &str) -> Self {
        Self { toast: String::from(item), seasonal_fruit: String::from("Free Juice") }
    }
}
