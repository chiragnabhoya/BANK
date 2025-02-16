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

fn print_account(account: Account) {
    println!("{:#?}", account);

}

fn print_holder(holder: String) {
    println!("{}", holder);
}

fn main() {
    let account = Account::new(1, "Alice".to_string());

    // print_holder(account.holder);
    print_account(account);

}
