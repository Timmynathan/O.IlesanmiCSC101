fn main() {
        //index   0   1   2   3   4   5   6  7
    let v = vec!['C','O','M','P','U','T','E','R'];

    let mut input1 = String::new();

    println!("Enter an index value between (0 - 7)");
    std::io::stdin().read_line(&mut input1).expect("Failed to read input");
    //index is from zero, so if its 1-7, the index os 0,1,2,3,4,5,6.
    let index:usize = input1.trim().parse().expect("Invalid input");

    //getting value at given index value
    let ch: char = v[index];

    println!("{} is the character for index [{}]\n",ch,index);
}
