fn main() {
    let arr = [1, 2, 3, 4, 5];

    // Get a slice containing [2, 3]
    let slice = &arr[1..3];

    println!("Slice: {:?}", slice);

    assert_eq!(slice, &[2, 3]);
}
