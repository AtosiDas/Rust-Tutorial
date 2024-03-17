//Option is another enum defined by the standard library.
// The Option type encodes the very common scenario in which a value could be something or it could be nothing.
//For example, if we request the first item in a non-empty list, we would get a value. If we request the first item in an empty list, we would get nothing.

/*As such, Rust does not have nulls, but it does have an enum that can encode the concept of a value being present or absent.*/
// This enum is Option<T>
/*The Option<T> enum is so useful that it’s even included in the prelude; we don’t need to bring it into scope explicitly. 
Its variants are also included in the prelude: we can use Some and None directly without the Option:: prefix. The Option<T> enum is still just a regular enum, and Some(T) and None are still variants of type Option<T>.*/

enum Option<T> {  //The <T> syntax is a feature of Rust and it’s a generic type parameter
    None,
    Some(T),
}

/*<T> means that the Some variant of the Option enum can hold one piece of data of any type, and that each concrete type that gets used in place of T makes the overall Option<T> type a different type.*/

let some_number = Some(5);
let some_char = Some("A");
let abosent_number: Option<i32> = None;
//The type of some_number is Option<i32>. The type of some_char is Option<char>,
//absent_number to be of type Option<i32>.
/*When we have a Some value, we know that a value is present and the value is held within the Some. When we have a None value, in some sense it means the same thing as null: we don’t have a valid value*/

let x: i8 = 5;
let y: Option<i8> =  Some(5);
let sum = x + y;
//Option<T> and T (where T can be any type) are different types, the compiler won’t let us use an Option<T> value as if it were definitely a valid value.