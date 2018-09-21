#[derive(Debug)]
pub struct Fortune {
	pub balance: f64,
}

impl Fortune{
	pub fn get_balance(self) -> f64{
		self.balance
	}
}