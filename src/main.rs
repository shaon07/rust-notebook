struct ReactAngle {
    width: u32,
    height: u32,
}

impl ReactAngle {
    // in methods first param will be by default self.
    fn area(&self) -> u32 {
        return self.width * self.height;
    }

    // now lets take others params
    fn area_with_unit(&self, unit: &str) -> u32 {
        let area: u32 = self.height * self.height;
        println!("The Area of the reactangle is {}{}", area, unit);
        return area;
    }
}

struct Square {
    width: u32,
    height: u32,
}

impl Square {
    // Associated Functions. its use to do something with the struct itself. and return Self type
    fn create(size: u32) -> Self {
        Self { width: size, height: size }
    }
}

fn main() {
    let rect_1: ReactAngle = ReactAngle { width: 40, height: 60 };
    println!("The area of the reactangle is {}", rect_1.area());
    rect_1.area_with_unit("KM");

    let sq: Square = Square::create(10);
    println!("The Dimension of the Square is width:{} and height:{}", sq.width, sq.height);
}
