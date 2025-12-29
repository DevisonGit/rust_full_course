fn main() {
    let mut _x = 5;
    let _s = &mut _x;
    *_s += 1;

    println!("value of _s: {}", _x);

    let mut account = BankAccount{
        owner: "Alice".to_string(),
        balace: 150.55
    };

    // Immutable borrow
    account.check_balance();

    // mutable borrow
    account.withdraw(50.55);

    account.check_balance();

}


struct BankAccount{
    owner: String,
    balace: f64,
}

impl BankAccount{
    fn withdraw(&mut self, amount: f64){
      println!("Withdrawing {} form account owned by {}", amount, self.owner);
      self.balace -= amount;
    }

    fn check_balance(&self){
        println!("Account owned by {} has a balance od {}", self.owner, self.balace);
    }

}