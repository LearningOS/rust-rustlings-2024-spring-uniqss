// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.

fn main() {
    let data = "Rust is great!".to_string();

    // &data 是不 take owership 的，但是要求必须只读不能 mute
    get_char(&data);

    can_use_by_ref_here(&data);

    string_uppercase(data);

    // can_use_by_ref_here(&data);

    // 也可以直接在函数里把一个 immutable 变成 mutable
    let s1 = String::from("hello");
    let mut s2 = s1;
    println!("{}", s2);
    // 并且还可以改变
    s2 = s2.to_uppercase();
    println!("{}", s2);
}

fn can_use_by_ref_here(data: &String) {
    println!("data:{}", data);
}

// Should not take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership 注意这个 mut !!! 它让一个不可变的变量，在里面变成了可变量
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();
    println!("{}", data);
}
