struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    let area = area(width1, height1);

    println!("[Main] The area of the rectangle is {} square pixels", area);

    main_tuple();
    main_struct();
}

fn main_tuple() {
    let rect = (30, 50);

    let area = area_tuple(rect);

    println!("[Main Tuple] The area of the rectangle is {} square pixels", area);
}

fn main_struct() {
    let rect = Rectangle{
        width: 30,
        height: 50,
    };

    let area = area_struct(&rect);

    println!("[Main Struct] The area of the rectangle is {} square pixels", area);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_struct(rect: &Rectangle) -> u32 { // we want to borrow the struct and not take ownership on it. That way main keeps ownership.
    rect.width * rect.height
}
