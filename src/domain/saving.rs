pub trait Saving{
	fn add(&mut self, amount:f64);
	fn withdraw(&mut self, amount:f64);
	fn get_balance(self)->f64;
}