 struct Laptops {
     brand:String,
     qty:u32,
     price:u32
 }

 impl Laptops {
    fn total_price(&self)->u32 {
    self.qty * self.price
    }
 }
 
 fn main() {

    let lap1 = Laptops {
        brand:String::from("HP"),
        qty:3,
        price:650_000
    };
    let lap2 = Laptops {
        brand:String::from("IBM"),
        qty:3,
        price:755_000
    };
    let lap3 = Laptops {
        brand:String::from("Toshiba"),
        qty:3,
        price:550_000
    };
    let lap4 = Laptops {
        brand:String::from("Dell"),
        qty:3,
        price:850_000
    };
    display(&lap1);
    display(&lap2);
    display(&lap3);
    display(&lap4);

    let total_cost = lap1.total_price() + lap2.total_price() + lap3.total_price() + lap4.total_price();
    println!("\nTotal cost is: {}", total_cost);
 }

 fn display(lap:&Laptops){
    println!("Brand is :{} \n quantity is :{} \n price is :{} \n total price is {}", lap.brand,lap.qty,lap.price,lap.total_price());
}