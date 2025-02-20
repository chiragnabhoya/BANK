#[derive(Debug)]
struct Account {
    id: u32,
    balance: i32,
    holder: String,
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account {
            id,
            holder,
            balance: 0,
        }
    }
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Bank {
        Bank {accounts: vec![]}
    }
}

fn print_account(account: &Account) {
    println!("{:#?}", account);
}

fn print_holder(holder: String) {
    println!("{}", holder);
}

fn change_account(account: &mut Account) {
    account.balance = 100;

    // println!("{}",account.holder);
}

fn main() {
    let mut account = Account::new(1, "Alice".to_string());

    change_account(&mut account);
    

    println!("{:#?}", account);

}
