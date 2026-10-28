 use std::io::Write;

 fn main() {

 let commissioners = vec!["Aigbogun Alamba Daudu",
 	"Murtala Afeez Bendu",
 	"Okorocha Calistus Ogbana",
 	"Adewale Jimoh Akanbi",
 	"Osazuwa Faith Etieye"];
 let ministry = vec!["Internal Affairs",
 	"Justice",
 	"Defense",
 	"Power & Steel",
 	"Petroleum"];
 let zone = vec!["South West",
 "North East",
 "South South",
 "South West",
 "South East"];

 for index in 0..commissioners.len() {
     let sum = commissioners[index].to_owned() + ministry[index] + zone[index];
     println!("{} {} {}", sum);
    }
 
 let mut file = std::fs::File::create("EFCC Merge.csv").expect("create failed");
    file.write_all(sum.as_bytes()).expect("write failed");

 println!("\nData written to file.");
}