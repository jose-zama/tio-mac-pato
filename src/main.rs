mod domain;

use domain::Fortune;

fn main(){
	let fortune = Fortune{
		balance : 50.32,
	};
	let balance = fortune.get_balance();
	println!("{}",balance);
}