fn main() {
    // an array of numbers
    let numbers = [1,2,3,4,5];
    println!("Original array = {:?}", numbers);

    // create a slice of 2nd and 3rd element
    let slice1 = &numbers[1..3];
    println!(" 2nd and 3rd elements sliced = {:?}", slice1);

     // omit the standard index
    let slice2 = &numbers[..3];
    // this means the slice starts from index 0 and goes up to index 3 (exclusive)
    println!("index 0 and index 3 sliced = {:?}", slice2);

    // omit the standard index
    let slice3 = &numbers[2..];
    // this means the slice starts from index 2 and goes up to index 5 (exclusive)
    println!("index 2 and index 5 sliced = {:?}", slice3);

    // omit the standard index
    // reference the whole array
    let slice4 = &numbers[..];
    // this means the slice starts from index 0 and goes up to index 5 (exclusive)
    println!("index 0 and index 5 sliced = {:?}", slice4);
}
