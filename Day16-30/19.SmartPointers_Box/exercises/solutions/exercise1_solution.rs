fn main() {
    // Allocating a large array on the heap
    let heap_array = Box::new([0; 1000]);

    println!("First element: {}", heap_array[0]);

    // The size of the variable `heap_array` is the size of the pointer (usually 8 bytes on 64-bit)
    // not 1000 * 4 bytes.
    let ptr_size = std::mem::size_of_val(&heap_array);
    println!("Size of Box pointer: {} bytes", ptr_size);
    assert_eq!(ptr_size, std::mem::size_of::<usize>());
}
