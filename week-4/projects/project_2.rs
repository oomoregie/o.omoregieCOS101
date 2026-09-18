 use std::io;

 fn main() {

     let mut input1 = String::new();
     let mut input2 = String::new();

 // Experience
     println!("Are you experienced?: ");
     io::stdin().read_line(&mut input1).expect("Not a valid string");
     let cleaned_input = input1.trim();

     if cleaned_input == "Yes" {
         println!("You may proceed {}", cleaned_input)
   }
     if cleaned_input == "No" {
     	 println!("Your Annual Incentive is N100,000");
     	 return;
   }

 // Age
     println!("Enter your age: ");
     io::stdin().read_line(&mut input2).expect("Not a valid string");
     let age:u8 = input2.trim().parse().expect("Not a valid number");

     if age >= 40 {
     	println!("Your Annual Incentive is N1,560,000");
     }
     else if age < 40 && age > 29 {
     	println!("Your Annual Incentive is N1,480,000");
     }
     else if age < 29 {
     	println!("Your Annual Incentive is N1,300,000");
     }
}