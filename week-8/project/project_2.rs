 fn main() {

    let candidates = vec![("Grace", 3),("Effiong", 7),("Peter", 10),("Chinedu", 5),("Amina", 12)];

    let mut highest_years = 0;
    let mut best_name = " ";

    for i in 0..candidates.len() {
        let (name, years) = candidates[i];
        if years > highest_years {
            highest_years = years;
            best_name = name;
        }
    }

    println!("{} has the highest experience with {} years.", best_name, highest_years);
}