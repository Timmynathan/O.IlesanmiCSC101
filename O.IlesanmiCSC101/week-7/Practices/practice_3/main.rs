//using the get() method
fn value(a:Option<&char>)
{
    println!("Element of vector {:?}",a);
}
fn main(){
//the index is    0 .  1 .  2 . 3 . 4 . 5  6 . 7 . 8
    let v = vec!['R','U','S','T','A','C','I','A','N'];

    let mut input1 = String::new();
    println!("\nInput any index value between (0-8)");
    std::io::stdin().read_line(&mut input1).expect("Failed to read input");
    let index:usize = input1.trim().parse().expect("Invalid input");

    let ch: Option<&char> = v.get(index);
    value(ch);
}