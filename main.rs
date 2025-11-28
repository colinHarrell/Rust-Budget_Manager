//use std::env;
use std::io;

fn main(){

println!("Welcome to Budget Manager!");
println!("Are you a new user? (y/n)");
let mut y_or_n = String::new();
io::stdin().read_line(&mut y_or_n).expect("Failed to read line");

if y_or_n.trim() == "n" {

println!("Enter username:");
let mut username = String::new();
io::stdin().read_line(&mut username).expect("Failed to read line");

let _trimmed_name_un = username.trim();
println!();

println!("Enter password: ");
let mut password = String::new();
io::stdin().read_line(&mut password).expect("Failed to read line");

let _trimmed_name_pw = password.trim();
println!();

println!("Welcome to your Budget Manager, {}!", _trimmed_name_un);

} else if y_or_n.trim() == "y" {
    println!("Create a new user account.");
}

}