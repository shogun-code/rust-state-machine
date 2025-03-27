mod balances;
mod system;

#[derive(Debug)]
pub struct Runtime {
    system: system::Pallet,
    balances: balances::Pallet,
}

impl Runtime {
    // Create a new instance of the main Runtime, by creating a new instance of each pallet.
    fn new() -> Self {
        Self {
            system: system::Pallet::new(),
            balances: balances::Pallet::new(),
        }
    }
}

fn main() {
    // Init balance
    let mut runtime = Runtime::new();
    runtime.balances.set_balance(&"alice".to_string(), 100);

    // start emulating a block
    runtime.system.inc_block_number();
    assert_eq!(runtime.system.block_number(), 1);

    // first transaction
    runtime.system.inc_nonce(&"alice".to_string());
    let _res = runtime.balances
        .transfer("alice".to_string(), "bob".to_string(), 30)
        .map_err(|err| eprintln!("{:?}", err));

    // second transaction
    runtime.system.inc_nonce(&"alice".to_string());
    let _res = runtime.balances
        .transfer("alice".to_string(), "charlie".to_string(), 20)
        .map_err(|err| eprintln!("{:?}", err));

    println!("Runtime: {:#?}", runtime);
}