use std::io::Read;
fn main(){
    let mut file = std::fs::File::open("welcome_message.txt")
    .unwrap();

    //create an empty string
    let mut contents = String::new();
    
    file.read_to_string(&mut contents).unwrap();
     
    print!("{}", contents);
}