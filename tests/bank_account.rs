extern crate tio_rico_mcpato;
use tio_rico_mcpato::domain::BankAccount;
use tio_rico_mcpato::domain::Saving;

#[test]
fn balance_should_be_equal_initial_value(){
	let account = BankAccount::new(String::from("1010101"),String::from("yep"),4.20);
	assert_eq!(4.20, account.get_balance());
}

#[test]
fn balance_should_be_equal_to_amount_added(){
	let mut account = BankAccount::new(String::from("1010101"),String::from("yep"),0.0);
	account.add(60.34);
	assert_eq!(60.34, account.get_balance());
}

#[test]
fn balance_should_be_equal_to_the_negative_amount_withdrawn(){
	let amount_to_withdrawn = 12343234.23;
	let mut account = BankAccount::new(String::from("treosk3o3"),String::from("my precious"), 0.0);
	account.withdraw(amount_to_withdrawn);
	assert_eq!(-amount_to_withdrawn,account.get_balance());
}