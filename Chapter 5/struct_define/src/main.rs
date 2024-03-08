//To define a struct, we enter the keyword struct and 
//name the entire struct. A struct’s name should describe the 
//significance of the pieces of data being grouped together. Then, inside curly brackets, 
//we define the names and types of the pieces of data, which we call fields.

//To use a struct after we’ve defined it, we create an instance of that struct by specifying concrete values for each of the fields. 

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
struct Color(i32, i32, i32); //Using Tuple Structs Without Named Fields to Create Different Types
struct Point(i32, i32, i32);


struct AlwaysEqual; //These are called unit-like structs because they behave similarly to ()

fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    let user3 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    //The syntax .. specifies that the remaining fields not explicitly set should have the same value as the fields in the given instance.
    /*we can no longer use user1 as a whole after creating user2 because the String in the username field of user1 was moved into user2. 
    If we had given user2 new String values for both email and username, and thus only used the active and sign_in_count values from user1, 
    then user1 would still be valid after creating user2.*/

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;
}
//We don’t have to specify the fields in the same order in which we declared them in the struct.
//Note that the entire instance must be mutable; Rust doesn’t allow us to mark only certain fields as mutable

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username, //we can also write this in shorthand only write "username"
        email: email,
        sign_in_count: 1,
    }
}
