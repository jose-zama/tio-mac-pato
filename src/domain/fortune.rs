#[derive(Debug)]
pub struct Fortune {
	balance: f64,
}

impl Fortune{

	pub fn new() -> Fortune{
		Fortune {balance: 0.0}
	}

	pub fn get_balance(self) -> f64{
		self.balance
	}

	pub fn set_balance(&mut self, balance: f64) {
		self.balance = balance;
	}
}