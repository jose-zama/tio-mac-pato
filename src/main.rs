mod domain;

use domain::Fortune;

fn main(){
	let fortune = Fortune::new();
	let balance = fortune.get_balance();
	println!("{}",balance);
}