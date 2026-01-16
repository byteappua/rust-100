fn main() {
    let t = (1, String::from("hello"));

    // 场景 1: 整体移动
    // let t2 = t;
    // println!("t.0: {}", t.0); // 编译错误！整个元组被移动了，因为其中包含非 Copy 元素。

    // 场景 2: 部分移动 (Partial Move)
    // 我们可以只移动其中的 String 字段
    let _s = t.1;

    // 此时 t.0 (i32) 是 Copy 类型且未被移动，仍然有效
    println!("t.0 is still valid: {}", t.0);

    // t.1 已经被移动了，无法访问
    // println!("t.1 is invalid: {}", t.1);
    // println!("t is invalid: {:?}", t); // 整个 t 也被视为部分失效

    // 对比：全 Copy 的元组
    let c = (1, 2);
    let _c2 = c;
    println!("c is still valid: {:?}", c); // 正常运行，因为 (i32, i32) 实现了 Copy
}
