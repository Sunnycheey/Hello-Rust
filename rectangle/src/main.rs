#[derive(Debug)]
struct Rectangle {
	width: u32,
	height: u32
}
fn main() {
    let rec = Rectangle {
        width: 10,
        height:20
    }; 
	println!("rec is: {:#?}", rec);
    println!("the area of rectangle is: {:#?}", area(&rec));
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
