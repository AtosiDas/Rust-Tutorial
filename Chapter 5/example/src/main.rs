//An example - calculate the area of rectangle using Struct
#[derive(Debug)]
struct Details {
    width: u32,
    height: u32,
}

fn main() {
    let width = 30;
    let height = 50;

    println!("The area of the rectangle is {} square pixels.",area(width,height));

    let rec1 = (40, 50);
    println!("The area of the rectangle is {} square pixels.",area1(rec1));

    let rec2 = Details {
        width: 20,
        height: 30,
    };
    //println!("The area of the rectangle is {} square pixels.",area(rec2.width, rec2.height));

    println!("The area of the rectangle is {} square pixels.",area2(&rec2));
    println!("The rectangle is {:?}", rec2); 
    //println!("The rectangle is {},"rec2); // This will create an error.
}
fn area(width: u32, height:u32) -> u32 {
    width * height
}
fn area1(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
fn area2(rectangle: &Details) -> u32 {
    rectangle.width * rectangle.height
}
