/*Methods are similar to functions: we declare them with the fn keyword and a name, 
they can have parameters and a return value, and they contain some code that’s run when the method is called from somewhere else. 
Unlike functions, methods are defined within the context of a struct, and their first parameter is always self, which represents the instance of the struct the method is being called on.*/

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {  //we start an impl (implementation) block for Rectangle. Everything within this impl block will be associated with the Rectangle type.
    fn area(&self) -> u32 {  //The &self is actually short for self: &Self. Within an impl block, the type Self is an alias for the type that the impl block is for. 
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }
    fn can_hold(&self, other: &Rectangle) -> bool { //This is a method which have more than one argument.
        self.area() > other.area()
    }
    fn square(size: u32) -> Self { //This is not a method.
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()  //we called the area function and passed rect1 as an argument
    );
    if rect1.width() {
        println!("The width {} of the rectangle is greater than 0",rect1.width);
    }
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width:60,
        height: 45,
    };
    println!("Can react1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can react1 hold rect3? {}", rect1.can_hold(&rect3));
}

/*Rust doesn’t have an equivalent to the -> operator; instead, Rust has a feature called automatic 
referencing and dereferencing. Calling methods is one of the few places in Rust that has this behavior.
when we call a method with object.something(), Rust automatically adds in &, &mut, or * so object matches the signature of the method.*/

//the following are the same:
//p1.distance(&p2);
//(&p1).distance(&p2);

//Each struct is allowed to have multiple impl blocks.

//<----------------Important---------------->
//1)Structs let us create custom types that are meaningful for our domain.
//2)By using structs, we can keep associated pieces of data connected to each other and name each piece to make our code clear.
//3)In impl blocks, we can define functions that are associated with our type, and methods are a kind of associated function that let us specify the behavior that instances of our structs have.