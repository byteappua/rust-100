fn main() {
    let s = String::from("hello");

    // 打印 s 在栈上的地址
    println!("Stack address of s: {:p}", &s);

    // 打印 s 指向的堆地址
    println!("Heap address of data: {:p}", s.as_ptr());

    // 发生 Move
    let s2 = s;

    // 打印 s2 在栈上的地址 (应该不同于 s)
    println!("Stack address of s2: {:p}", &s2);

    // 打印 s2 指向的堆地址 (应该完全相同)
    println!("Heap address of s2 data: {:p}", s2.as_ptr());

    // 结论：
    // Stack 地址变了 -> 栈上发生了拷贝
    // Heap 地址没变 -> 没有拷贝堆数据，只是转移了指针的所有权
}
