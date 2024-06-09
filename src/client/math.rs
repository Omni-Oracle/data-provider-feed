pub trait RoundUp {
    fn round_up(self, places: u8) -> Self;
}

impl RoundUp for f64 {
    fn round_up(self, places: u8) -> Self {
        let factor = 10f64.powi(places as i32);
        (self * factor).ceil() / factor
    }
}
