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
