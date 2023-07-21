use std::io::Write;

fn main() {

    let announce = "Rust File Input & Output";
    let dept = "Department of Computer science";

    let mut file = std::fs::File::create("data.txt")
    .expect("unable to create");

    file.write_all(b"Welcome to Rust Programming")
    .expect("unable to write");
    
    file.write_all(announce.as_bytes())
    .expect("write failed");

    file.write_all(dept.as_bytes())
    .expect("write failed");
    
    println!("\nData written to file.");

}