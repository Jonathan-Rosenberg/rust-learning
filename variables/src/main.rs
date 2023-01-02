fn main() {
    let mut x = 4;
    println!("The value is: {x}");
    x = 6;
    println!("The value is: {x}");
    array_func();
}

fn array_func() -> [i32; 5] {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    return a;
}

fn array_func_2() -> [i32; 5] {
    let a: [i32; 5] = [3; 5];
    return a;
}