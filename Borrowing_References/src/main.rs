// References and Borrowing
// Safety and Performance
// Borrowing and references are powerful concepts

// Understanding References
// Immutable References
// Mutable References
// Create Reference by adding "&"
fn main() {
    let _x = 5;
    let _r = &_x;
    println!("Value of _x : {}", _x);
    println!("Value of _r : {}", _r);


    let mut _y = 5;
    let _t = &mut _y;

    *_t += 1;
    *_t -= 3;
    println!("Value of _x : {}", _y);

    let mut account = BankAccount{
        owner: "John".to_string(),
        balance: 150.33,
    };
    // Immutable borrow to check the balance
    account.check_balance();

    // Mutable borrow to withdraw

    account.withdraw(50.5);

    // Immutable borrow to check the balance
    account.check_balance();

    // Mutable borrow to deposit
    account.deposit(1900.45);

    // Immutable borrow to check the balance
    account.check_balance();


}

struct BankAccount{
    owner: String,
    balance: f64,
}

impl BankAccount{
    fn withdraw(&mut self, amount: f64){
        println!("Withdraw {} from account owned by {}", amount, self.owner);
        self.balance -= amount;
    }

    fn check_balance(&self){
        println!("Account owned by {} has a balance of {}", self.owner, self.balance);
    }

    fn deposit(&mut self, amount: f64){
        println!("Deposit {} from account owned by {}", amount, self.owner);
        self.balance += amount;
    }

}