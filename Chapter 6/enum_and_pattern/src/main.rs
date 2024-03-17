//Enums give us a way of saying a value is one of a possible set of values.

enum IpAddrKind {
    V4,
    V6,
}  //IpAddrKind is now a custom data type that we can use elsewhere in our code.

let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
//Now both values IpAddrKind::V4 and IpAddrKind::V6 are of the same type: IpAddrKind. 

fn route(ip_kind: IpAddrKind) {

}

route(four);
route(six);

//Now we want to store the Ip address and the kind of the ip address. So we have to use struct.

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
};
let office = IpAddr {
    kind: IpAddrKind::V6,
    address: String::from("::1"),
};

enum IpAddr1 {
    V4(String),
    V6(String),
}
let home1 = IpAddr1::V4(String::from("127.0.0.1"));
let office1 = IpAddr1::V6(String::from("::1"));

//IpAddr::V4() is a function call that takes a String argument and returns an instance of the IpAddr type.

enum IpAddr2 {
    V4(u8, u8, u8, u8),
    V6(String),
}
let home2 = IpAddr2::V4(127,0,0,1);
let office2 = IpAddr2::V6(String::from("::1"));

//We can also use struct inside an enum
struct Ipv4 {

}
struct Ipv6 {

}
enum IpAddr3 {
    V4(Ipv4),
    V6(Ipv6),
} //This code illustrates that we can put any kind of data inside an enum variant: strings, numeric types, or structs, for example.
//We can also include another enum.

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

//There is one more similarity between enums and structs: just as we’re able to define methods on structs using impl, we’re also able to define methods on enums. 
//Here is a method named "call" that we define.
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}
let m = Message::Write(String::from("hello"));
m.call();

//<----------Advantages of using Enums --------------------->
/*
1) Each variant can have different types and amounts of associated data. 

*/