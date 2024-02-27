//Generate the nth Fibonacci number.

use std::io;

fn main() {
    println!("Enter the value of n");
    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Failed");
    let n : u32 = n.trim().parse().expect("Failed");
    println!("You entered {n}");
    let mut n1 = 1;
    let mut n2 = 1;
    let mut count = 3;
    let mut dummy = 0;
    if n == 1{
        println!("{n1}");  
    }
    else if n == 2{
        println!("{n1}");
        println!("{n2}");
    }else{
        println!("{n1}");
        println!("{n2}");
        while count <= n {
            dummy = n1;
            n1 = n2;
            n2 = dummy + n2;
            println!("{n2}");
            count += 1;
        }
        
    }
}
    