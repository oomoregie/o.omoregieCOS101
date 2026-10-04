 use std::io;

 fn main() {
    println!("Welcome to the Restaurant!");
    println!("Menu:");
    println!("P: Poundo Yam/Edinkaiko Soup - N3,200");
    println!("F: Fried Rice & Chicken - N3,000");
    println!("A: Amala & Ewedu Soup - N2,500");
    println!("E: Eba & Egusi Soup - N2,000");
    println!("W: White Rice & Stew - N2,500");

    let mut code = String::new();
    let mut qty = String::new();

    println!("Enter food code (P/F/A/E/W): ");
    io::stdin().read_line(&mut code).expect("Not a valid string");
    let code = code.trim();

    println!("Enter quantity: ");
    io::stdin().read_line(&mut qty).expect("Not a valid string");
    let qty:u16 = qty.trim().parse().expect("Not a valid number");

    let price = match code {
         "P" => 3200,
         "F" => 3000,
         "A" => 2500,
         "E" => 2000,
         "W" => 2500,
         _ => {
              println!("Invalid code!");
              return;
            }
        };

    let total = price * qty;
    println!("Total before discount: N{}", total);

    let final_total = if total > 10000 {
        let discount = (total as f32) * 0.05;
        total as f32 - discount
    } else {
       total as f32
    };

    println!("Final  total: N{}", final_total);
}