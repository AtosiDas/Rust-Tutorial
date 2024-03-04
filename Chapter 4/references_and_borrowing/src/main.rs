fn main() {
    let s1 = String::from("Atosi Das");  // memory allocation

    let len = calculate_length(&s1);  //references
    //The &s1 syntax lets us create a reference that refers to the value of s1 but does not own it.
    println!("The length of '{}' is {}.", s1, len);
    let mut s = String::from("Hello");
    println!("The actual value is {}",s);
    append(&mut s);
    println!("The new value is {}",s);
    let r1 = &mut s;  //first mutable borrow occurs here
    //let r2 = &mut s; // second mutable borrow occurs here -- we can not borrow twice at the same time.
    /*The first mutable borrow is in r1 and must last until it’s used in the println!, but between 
    the creation of that mutable reference and its usage, we tried to create another mutable reference 
    in r2 that borrows the same data as r1.*/
    println!("{}",r1);
    {
        let r2 = &mut s; // But we can borrow here
        println!("{}",r2);
    }
    let mut s2 = String::from("Atosi");
    let x1 = &s2;
    let x2 = &s2;
    //let x3 = &mut s2;
    //println!("{}, {} ,{}",x1,x2,x3);  //Rust enforces a similar rule for combining mutable and immutable references.
    println!("{} , {}",x1,x2);
    //We also cannot have a mutable reference while we have an immutable one to the same value.
    
    let x3 = &mut s2;
    println!("x3 = {}",x3);

    //Dangling references
    //let reference_to_nothing = dangle(); //this function's return type contains a borrowed value, but there is no value for it to be borrowed from
    let reference_value = no_dangle();
    println!("{}", reference_value);
}
//Just as variables are immutable by default, so are references. We’re not allowed to modify something we have a reference to.

// fn dangle() -> &String {  //dangle returns a reference to a string
//     let s = String::from("Hello World");  // a new string s is created
//     &s // This function returns a reference to the string s
// } //Here, s goues out of scope, and is dropped. Its memory goes away.
//Danger

fn no_dangle() ->String {
    let s = String::from("Atosi Das");
    s
}

fn append(s: &mut String) {
    s.push_str(" world!");
}
fn calculate_length(s: &String) -> usize {
    s.len()
}

//The opposite of referencing by using & is dereferencing, which is accomplished with the dereference operator, *.
/* Because it does not own it, the value it points to will not be dropped when the reference stops being used.*/
//By default, the references are immutable.
//Mutable references have one big restriction: if we have a mutable reference to a value, we can have no other references to that value.

/* The benefit of having this restriction is that Rust can prevent data races at compile time. A data race is similar to a race condition 
and happens when these three behaviors occur:

1)Two or more pointers access the same data at the same time.
2)At least one of the pointers is being used to write to the data.
3)There’s no mechanism being used to synchronize access to the data.*/

//<--------------Dangling References------------->

/*Dangling pointer — a pointer that references a location in memory that may have been given to someone else—by 
freeing some memory while preserving a pointer to that memory.*/


//<-*-*-*-*-*-*-* RECAP *-*-*-*-*-*-*-*-*->

// we’ve discussed about references:

// (1) At any given time, you can have either one mutable reference or any number of immutable references.
// (2) References must always be valid.