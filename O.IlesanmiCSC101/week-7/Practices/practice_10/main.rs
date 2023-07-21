fn main() {
    // an array of numbers
           //index 0  1  2   3  4
    let numbers = [1, 2, 3, 4, 5];
    println!("Original array = {:?}", numbers);

    //create a slice of the 2nd and 3rd element, 2 and 3
    let slice1 = &numbers[1..3]; // 3 is exluded so its 1,2
    println!("2nd and 3rd elements sliced = {:?}", slice1);

    //omit the start index
    let slice2 = &numbers[..3];
    // This means that the slice starts from 
    //index 0 to index 3(exclusive), so 0 to 2
    println!("index 0 to index 3 sliced ={:?}", slice2);

    //omitting the end index
    let slice3 = &numbers[2..];
    //this means the slice sarts from index 2 to index 5(exclusive)
    // so 2 to 4
    println!("index 2 to index 5 sliced = {:?}", slice3);
    
    //omit BOTH start and end index
    //it refrences the entire array
    let slice4 = &numbers[..];
    //the slice starts from index 0 to index 5(exclusive);
    //so 1 to 5
    println!("index 0 to index 5 sliced = {:?}", slice4);
}