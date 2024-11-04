mod balances;

use balances::Pallet;
fn main() {
    let mut pallet = Pallet::new();
    pallet.set_balance(&"Matheus".to_string(), 1000);

    let balance = pallet.get_balance(&"Matheus".to_string());
    println!("Balance: {}", balance);
    println!("Hello, world! WEB3DEV Rocks");
}
