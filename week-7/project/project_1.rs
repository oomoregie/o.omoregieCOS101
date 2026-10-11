 use std::io;

 fn trampezium_area(h: f64, b1: f64, b2: f64) -> f64 {
      (h / 2.0) * (b1 + b2)
 }

 fn rhombus_area(d1: f64, d2: f64) -> f64 {
      0.5 * d1 * d2
 }

 fn parallelogram_area(base: f64, altitude: f64) -> f64 {
      base * altitude
 }

 fn cube_area(side: f64) -> f64 {
      6.0 * side * side
 }

 fn cylinder_volume(radius: f64, height: f64) -> f64 {
      std::f64::consts::PI * radius * radius * height
 }

 fn main() {
     println!("Shape Calculator");
     println!("Choose a shape:");
     println!("1. Trampezium (area)");
     println!("2. Rhombus (area)");
     println!("3. Parallelogram (area)");
     println!("4. Cube (surface area)");
     println!("5. Cylinder (volume)");

     let mut choice = String::new();
     io::stdin().read_line(&mut choice).expect("Failed to read input");
     let choice: i32 = choice.trim().parse().expect("Invalid input");

     match choice {
          1 => {
              println!("Enter height, base1, base2:");
              let mut h = String::new();
              let mut b1 = String::new();
              let mut b2 = String::new();
              io::stdin().read_line(&mut h).expect("Failed to read input");
              io::stdin().read_line(&mut b1).expect("Failed to read input");
              io::stdin().read_line(&mut b2).expect("Failed to read input");
              let h: f64 = h.trim().parse().expect("Invalid input");
              let b1: f64 = b1.trim().parse().expect("Invalid input");
              let b2: f64 = b2.trim().parse().expect("Invalid input");
              println!("Area = {}", trampezium_area(h, b1, b2));
          }

          2 => {
              println!("Enter diagonal1, diagonal2:");
              let mut d1 = String::new();
              let mut d2 = String::new();
              io::stdin().read_line(&mut d1).expect("Failed to read input");
              io::stdin().read_line(&mut d2).expect("Failed to read input");
              let d1: f64 = d1.trim().parse().expect("Invalid input");
              let d2: f64 = d2.trim().parse().expect("Invalid input");
              println!("Area = {}", rhombus_area(d1, d2));
          }

          3 => {
              println!("Enter base, altitude:");
              let mut base = String::new();
              let mut alt = String::new();
              io::stdin().read_line(&mut base).expect("Failed to read input");
              io::stdin().read_line(&mut alt).expect("Failed to read input");
              let base: f64 = base.trim().parse().expect("Invalid input");
              let alt: f64 = alt.trim().parse().expect("Invalid input");
              println!("Area = {}", parallelogram_area(base, alt));
          }

          4 => {
              println!("Enter side length:");
              let mut side = String::new();
              io::stdin().read_line(&mut side).expect("Failed to read input");
              let side: f64 = side.trim().parse().expect("Invalid input");
              println!("Surface Area = {}", cube_area(side));
          }

          5 => {
              println!("Enter radius, height:");
              let mut r = String::new();
              let mut h = String::new();
              io::stdin().read_line(&mut r).expect("Failed to read input");
              io::stdin().read_line(&mut h).expect("Failed to read input");
              let r: f64 = r.trim().parse().expect("Invalid input");
              let h: f64 = h.trim().parse().expect("Invalid input");
              println!("Volume = {}", cylinder_volume(r, h));
          }
          _=> println!("Invalid choice!"),
    }
}