fn main() {
    let s1 = String::from("Hello");
    let s2 = take_and_return(s1);
    println!("{}", s2);
}

fn take_and_return(mut s: String) -> String {
    s.push_str(" World");
    s // 返回所有权
}
