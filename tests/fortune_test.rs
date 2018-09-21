extern crate tio_rico_mcpato;
use tio_rico_mcpato::domain::Fortune;

#[test]
fn get_balance_should_return_balance() {
	let fortune = Fortune::new();
	assert_eq!(0.0, fortune.get_balance());
}

#[test]
fn set_balance_should_set_balance_to_specified_value() {
    let mut fortune = Fortune::new();
    fortune.set_balance(12.0);
    assert_eq!(12.0, fortune.get_balance());
}