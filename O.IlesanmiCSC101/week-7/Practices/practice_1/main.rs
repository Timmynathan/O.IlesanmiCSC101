fn main() {
    //using the Vec::new() method
    let v : Vec<i64> = Vec::new();

    //printing the size of vector
    println!("\nThe length of the vector is: {}",v.len());

    //using the Macro method
    let v = vec!["Grace","Effiong","Basil","Kareem","Susan"];

    //printing the size of the vector
    println!("\nThe length of the vector is: {}",v.len());
}
