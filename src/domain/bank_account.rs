use domain::saving::Saving;

pub struct BankAccount{
	account_id:String,
	alias:String,
	balance:f64,
}

impl Saving for BankAccount{
	fn add(&mut self,amount:f64){
		self.balance += amount;
	}
	fn withdraw(&mut self,amount:f64){
		self.balance -= amount;
	}
	fn get_balance(self)->f64{
		self.balance
	}
}

impl BankAccount{
	pub fn new(account_id:String, alias:String, balance:f64) -> BankAccount{
		BankAccount{
			account_id,
			alias,
			balance
		}
	}
	pub fn get_alias(self)->String{
		self.alias
	}
	pub fn get_account_id(self)->String{
		self.account_id
	}
}