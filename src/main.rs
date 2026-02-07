#[derive(Debug)]
struct Reactangle {
    width: u32,
    height: u32,
}
fn main() {
    // Lets understand why we need to use struct. to solve a problem

    // 1. First we will solve a problem without using struct

    // Problem: Find the Area of a Reactangle
    let width: u32 = 100;
    let height: u32 = 150;
    // here is the problem if the problem is a huge calucaltion and may take more params. then we might pass a wrong param at wrong position
    let result: u32 = reactangle_area_without_struct(width, height);
    println!("Area of the Reactangle is: {}", result);

    // so lets try to solve it with a tuple
    let rect: (u32, u32) = (100, 200);
    // here is the problem is we dont which position value is width and which one is height. lets say we mistakenly pass width param value as height so on.
    let result: u32 = reactangle_area_without_struct(rect.0, rect.1);
    println!("Area of the Reactangle is: {}", result);

    // so lets try to solve it with struct.
    let reactangle: Reactangle = Reactangle {
        height: 144,
        width: 135,
    };
    let result: u32 = reactangle_area_struct(reactangle);
    println!("Area of the Reactangle is: {}", result);

    // so lets look at the struct way to solve the problem. its so much clean and understandable.
    // we can also solve this using only taking the reference of the reactangle.

    // so now solve it with taking reference instead of taking the full ownership.

    let rect: Reactangle = Reactangle { width: 260, height: 300 };
    let res: u32 = reactangle_area_struct_with_borrowing(&rect);
    println!("The width:{}, height:{} and area: {}", rect.width, rect.height, res);
}

fn reactangle_area_without_struct(width: u32, height: u32) -> u32 {
    let area: u32 = width * height;
    return area;
}

fn reactangle_area_struct(reactangle: Reactangle) -> u32 {
    let area: u32 = reactangle.width * reactangle.height;
    return area;
}

fn reactangle_area_struct_with_borrowing(reactangle: &Reactangle) -> u32 {
    let area: u32 = reactangle.width * reactangle.height;
    return area;
}
