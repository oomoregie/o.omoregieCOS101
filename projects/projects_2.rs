fn main() {
	let qty_toshiba:f64 = 2.0;
	let qty_mac:f64 = 1.0;
	let qty_hp:f64 = 3.0;
	let qty_dell:f64 = 3.0;
	let qty_acer:f64 = 1.0;

	let amt_toshiba:f64 = 450000.0;
	let amt_mac:f64 = 1500000.0;
	let amt_hp:f64 = 750000.0;
	let amt_dell:f64 = 2850000.0;
	let amt_acer:f64 = 250000.0;

	let total_toshiba:f64 = qty_toshiba * amt_toshiba;
    let total_mac:f64 = qty_mac * amt_mac;
    let total_hp:f64 = qty_hp * amt_hp;
    let total_dell:f64 = qty_dell * amt_dell;
    let total_acer:f64 = qty_acer * amt_acer;

    // sum
    let sum = total_toshiba + total_mac + total_hp + total_dell + total_acer;
    println!("Sum is {}", sum);

    // average
    let sum_qty = qty_toshiba + qty_mac + qty_hp + qty_dell + qty_acer;
    let average = sum / sum_qty;
    println!("Average is{}", average);
}