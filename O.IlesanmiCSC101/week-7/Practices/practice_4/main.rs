fn main() {
    //Name Vector
    let name = vec!["Mary","Sam","Sally","Greg","Ade","Mark","June","Ife"];

    //Age vectorrrr
    let age = vec![16,17,19,22,20,21,18,23];
    println!("\nAge allocation:\n");

    //loop to iterate elements in vector
    for i in 0..age.len()
    {
        println!("{} is {} years old\n", name[i],age[i]);
    }
}
