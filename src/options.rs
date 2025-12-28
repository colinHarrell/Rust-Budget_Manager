  use std::io::{self, stdin};
  //use serde_json::Value;
  use crate::UserDB;
  use std::io::Write;

  //1. View Balances from individual accounts
  pub fn view_balances(username: &str, users: &UserDB){
    if let Some(user) = users.users.get(username) {
        println!("Balances for {}:", username);

        for (account, balance) in &user.accounts {
            println!("  {}: ${}", account, balance);
        }
    } else {
        println!("User '{}' not found.", username);
    }
  }

  //2. View Total of all accounts
  pub fn view_totals(username: &str, users: &UserDB){
    if let Some(user) = users.users.get(username) {
        let total: f32 = user.accounts.values().sum();
        println!("Total balance: ${}", total);
    } else {
        println!("User '{}' not found.", username);
    }

  }

  //3. Add Account and Balance
  pub fn add_account_and_balance() -> (String, f32) {
    println!("Enter account type (eg checkings, savings, credit, debt, etc):");
    let mut account_type = String::new();
    stdin().read_line(&mut account_type).unwrap();
    let account_type = account_type.trim().to_string();

    println!("Enter initial balance:");
    let mut balance = String::new();
    stdin().read_line(&mut balance).unwrap();
    let balance: f32 = balance.trim().parse().unwrap();

    println!("Account added successfully.");

    (account_type, balance)
  }

  //4. Deposit Money into Account
  pub fn deposit(users: &mut UserDB, username: &str) {
    println!("Enter account to deposit into:");
    let mut account = String::new();
    stdin().read_line(&mut account).unwrap();
    let account = account.trim();

    if let Some(user) = users.users.get_mut(username) {
        if let Some(current_balance) = user.accounts.get_mut(account) {
            println!("Enter amount to deposit:");
            let mut amount_str = String::new();
            stdin().read_line(&mut amount_str).unwrap();
            if let Ok(amount) = amount_str.trim().parse::<f32>() {
                *current_balance += amount;
                println!("Deposit successful. New balance: ${}", current_balance);
                users.save();
            } else {
                println!("Invalid amount.");
            }
        } else {
            println!("Account '{}' not found.", account);
        }
    } else {
        println!("User not found.");
    }
  }

  //5. Withdraw Money from Account
  pub fn withdraw(users: &mut UserDB, username: &str) {
    println!("Enter account to withdraw from:");
    let mut account = String::new();
    stdin().read_line(&mut account).unwrap();
    let account = account.trim();

    if let Some(user) = users.users.get_mut(username) {
        if let Some(current_balance) = user.accounts.get_mut(account) {
            println!("Enter amount to withdraw:");
            let mut amount_str = String::new();
            stdin().read_line(&mut amount_str).unwrap();
            if let Ok(amount) = amount_str.trim().parse::<f32>() {
                *current_balance -= amount;
                println!("Withdraw successful. New balance: ${}", current_balance);
                users.save();
            } else {
                println!("Invalid amount.");
            }
        } else {
            println!("Account '{}' not found.", account);
        }
    } else {
        println!("User not found.");
    }
  }

  //6. Remove Account
  pub fn remove_account(users: &mut UserDB,  username: &str){
    println!("Which account would you like to remove?");
    let mut account = String::new();
    stdin().read_line(&mut account).unwrap();
    let account = account.trim();

    if let Some(user) = users.users.get_mut(username) {
        if user.accounts.remove(account).is_some() {
            println!("Account '{}' removed successfully.", account);
            users.save();
        } else {
            println!("Account '{}' not found.", account);
        }
    } else {
        println!("User not found.");
    }
  }

  //7. Transfer Money between your Accounts
  pub fn internal_transfer(users: &mut UserDB, username: &str){
    println!("Which account would you like to transfer from?");
    let mut from_account = String::new();
    stdin().read_line(&mut from_account).unwrap();
    let from_account = from_account.trim();

    println!("Which account would you like to transfer to?");
    let mut to_account = String::new();
    stdin().read_line(&mut to_account).unwrap();
    let to_account = to_account.trim();

    println!("How much would you like to transfer?");
    let mut amount_str = String::new();
    stdin().read_line(&mut amount_str).unwrap();

    if let Ok(amount) = amount_str.trim().parse::<f32>() {
        if let Some(user) = users.users.get_mut(username) {
            if let (Some(from_balance), Some(to_balance)) = (user.accounts.get(from_account), user.accounts.get(to_account)) {
                if *from_balance >= amount && amount > 0.0 {
                    *user.accounts.get_mut(from_account).unwrap() -= amount;
                    *user.accounts.get_mut(to_account).unwrap() += amount;
                    println!("Transferred ${} from {} to {}.", amount, from_account, to_account);
                    users.save();
                } else if amount <= 0.0 {
                    println!("Invalid amount. Must be positive.");
                } else {
                    println!("Insufficient funds in {} account.", from_account);
                }
            } else {
                println!("One or both accounts not found.");
            }
        } else {
            println!("User not found.");
        }
    } else {
        println!("Invalid amount.");
    }
  }

  //8. Send Money to another User
  pub fn send_money(users: &mut UserDB, from_username: &str){
    println!("Which account would you like to send from?");
    let mut from_account = String::new();
    stdin().read_line(&mut from_account).unwrap();
    let from_account = from_account.trim();

    println!("Who would you like to send money to?");
    let mut recipient = String::new();
    stdin().read_line(&mut recipient).unwrap();
    let recipient = recipient.trim();

    println!("Which account type for the recipient (e.g., checkings, savings)?");
    let mut to_account = String::new();
    stdin().read_line(&mut to_account).unwrap();
    let to_account = to_account.trim();

    println!("How much would you like to send?");
    let mut amount_str = String::new();
    stdin().read_line(&mut amount_str).unwrap();

    if let Ok(amount) = amount_str.trim().parse::<f32>() {
        // First, check if sender exists and has sufficient funds
        let from_balance = users.users.get(from_username)
            .and_then(|u| u.accounts.get(from_account))
            .copied();

        match from_balance {
            Some(balance) if balance >= amount && amount > 0.0 => {
                // Check if recipient exists
                if let Some(to_user) = users.users.get_mut(recipient) {
                    // Add to recipient's account
                    let to_balance = to_user.accounts.entry(to_account.to_string()).or_insert(0.0);
                    *to_balance += amount;
                    // Subtract from sender's account
                    if let Some(from_user) = users.users.get_mut(from_username) {
                        *from_user.accounts.get_mut(from_account).unwrap() -= amount;
                    }
                    println!("Sent ${} from your {} to {}'s {}.", amount, from_account, recipient, to_account);
                    users.save();
                } else {
                    println!("Recipient '{}' not found.", recipient);
                }
            }
            Some(_) => {
                println!("Insufficient funds in {} account.", from_account);
            }
            None => {
                println!("Account '{}' not found.", from_account);
            }
        }
    } else {
        println!("Invalid amount.");
    }
  }

  //9 does nothing, just to exit

  //10. Delete Account
  pub fn delete_account(db: &mut UserDB, username: &str) {
    print!("Are you sure you want to delete your account? (y/n): ");
    io::stdout().flush().unwrap();

    let mut confirmation = String::new();
    io::stdin().read_line(&mut confirmation).unwrap();

    if confirmation.trim().eq_ignore_ascii_case("y") {
        if db.users.remove(username).is_some() {
            println!("Account deleted successfully.");
        } else {
            println!("User not found.");
        }
    }
}

