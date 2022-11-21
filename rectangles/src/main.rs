// fn main() {
//     let width1 = 30;
//     let height1 = 50;

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(width1, height1)
//     );
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// ---------- Using Tuples ----------
// fn main() {
//     let rect1 = (30, 50);
//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(rect1)
//     );
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// ----------- Using Struct ---------
// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(&rect1)
//     );
//     println!("{:#?}", rect1);
//     println!("{:?}", rect1);
//     dbg!(area(&rect1));
//     dbg!(rect1);
// }

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }


// --- Using Struct and Function ---
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    // println!("{:#?}", rect1);
    // println!("{:?}", rect1);
    // dbg!(rect1.area());
    // dbg!(rect1);

    let rect2 = Rectangle::square(20);

    let rect3 = Rectangle {
        width: 40,
        height: 10,
    };

    dbg!(rect1.can_hold(&rect2));
    dbg!(rect1.can_hold(&rect3));
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    fn square(size: u32) -> Self {
        Self {
            height: size,
            width: size,
        }
    }
}
