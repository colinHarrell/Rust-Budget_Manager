pub mod options{
  use std::io::{self, stdin};

  //1. View Balances from individual accounts
  pub fn view_balances(){
    
  }

  //2. View Total of all accounts (before & after debt)
  pub fn view_totals(){

  }

  //3. Add Account and Balance
  pub fn add_account_and_balance(){
    println!("Enter account type: (e.g., Checking, Savings, Credit Card, etc)");
    let mut account_type = String::new();
    stdin().read_line(&mut account_type).expect("Failed to read line");

    println!("Enter initial balance: ");
    let mut balance = String::new();
    stdin().read_line(&mut balance).expect("Failed to read line");
    let balance: f32 = balance.trim().parse().expect("Please enter a valid number");

    if balance < 0.0 {
      println!("Warning: You CAN NOT create an account with a negative balance");
    }else{
    println!("Account added successfully.");
    println!("Your {} account has been created with a balance of ${}.", account_type.trim(), balance);
  }
  }

  //4. Deposit Money into Account
  pub fn deposit(){

  }

  //5. Withdraw Money from Account
  pub fn withdraw(){
    
  }

  //6. Remove Account
  pub fn remove_account(){

  }

  //7. Transfer Money between your Accounts
  pub fn internal_transfer(){

  }

  //8. Send Money to another User
  pub fn send_money(){

  }

  //9 does nothing, just to exit

  //10. Delete Account
  pub fn delete_account(){

  }

}