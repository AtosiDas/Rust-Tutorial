fn main() {  
    println!("Hello, world!"); // s is not valid here, it's not yet declared
    //A scope is the range within a program for which an item is valid.
    //The variable s refers to a string literal. The variable is valid from the point at which it’s declared until the end of the current scope.
    let s ="Atosi";  // s is valid from this point forward
    // do stuff with s

    //When we call String::from, its implementation requests the memory it needs. 
    let mut s1 = String::from("Welcome"); //The double colon :: operator allows us to namespace this particular from function under the String type rather than using some sort of name like string_from.
    //Rust takes a different path: the memory is automatically returned once the variable that owns it goes out of scope.
    //Rust calls a special function for us. This function is called drop, and it’s where the author of String can put the code to return the memory.

    //println!("{}", s1);
    println!("{s1}");
    s1.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s1); // This will print `Hello, world!`

    let x = 5;
    let y = x;
    // Integers are simple values with a known, fixed size, and these two 5 values are pushed onto the stack.
    //let s3 = String::from("hello");
    //let s2 = s3;
    /*Here both data pointers of s2 and s3 are pointing to the same location. 
    This is a problem: when s2 and s3 go out of scope, they will both try to free the same memory. 
    This is known as a double free error and is one of the memory safety bugs we mentioned previously. 
    Freeing memory twice can lead to memory corruption, which can potentially lead to security vulnerabilities.*/
    
    //println!("{}", s3);  We get error because after the allocation, there is no value of s3. So, in Rust s3 is not valid after the assignment.

    /*In addition, there’s a design choice that’s implied by this: Rust will never automatically create “deep” copies of your data. 
    Therefore, any automatic copying can be assumed to be inexpensive in terms of runtime performance.*/
    //If we do want to deeply copy the heap data of the String, not just the stack data, we can use a common method called clone.

    let msg = String::from("Hello Atosi Das");
    let msg1 = msg.clone();  //Here deeplycopy is not done.
    println!("s1={}, s2={}",msg, msg1);


    let m = 15;
    let n = m;
    println!("m = {} and n = {}", m, n);
    /*But this code seems to contradict what we just learned: we don’t have a call to clone, but x is still valid and wasn’t moved into y.
    The reason is that types such as integers that have a known size at compile time are stored entirely on the stack, so copies of the actual values are quick to make.*/

    let s4 = String::from("hello Atosi Das");  // s comes into scope

    //takes_ownership(s4);             // s's value moves into the function...
                                  // ... and so is no longer valid her

    let (s4, len) = calculate_length(s4);
    println!("The length of {} is {}", s4, len);                              
    let x1 = 5;                      // x comes into scope

    makes_copy(x1);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

    //println!("{s4},{x1}");


    let ss1 = gives_ownership();         // gives_ownership moves its return
                                        // value into ss1

    let ss2 = String::from("hello");     // ss2 comes into scope

    let ss3 = takes_and_gives_back(ss2);  // ss2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into ss3

}  // this scope is now over, and s is no longer valid.
// Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
// happens. s1 goes out of scope and is dropped.

fn calculate_length(s: String) -> (String, usize){
    let length = s.len();
    (s, length)
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
fn gives_ownership() -> String {             // gives_ownership will move its
    // return value into the function
    // that calls it
    let some_string = String::from("yours"); // some_string comes into scope
    some_string                              // some_string is returned and
    // moves out to the calling
    // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
             // scope
    a_string  // a_string is returned and moves out to the calling function
}




//Ownership is a set of rules that govern how a Rust program manages memory.
//Rust uses a third approach: memory is managed through a system of ownership with a set of rules that the compiler checks.

//<------------ Stack an Heap --------------->

//************STACK**************

/*Both the stack and the heap are parts of memory available to your code to use at runtime, but they are structured in different ways.*/

//The stack stores values in the order it gets them and removes the values in the opposite order. This is referred to as last in, first out.
//Adding data is called pushing onto the stack, and removing data is called popping off the stack.
// All data stored on the stack must have a known, fixed size.

//Data with an unknown size at compile time or a size that might change must be stored on the heap instead.

//***************HEAP*****************
//when we put data on the heap, you request a certain amount of space. The memory allocator finds an empty spot in the heap that is big enough, 
//marks it as being in use, and returns a pointer, which is the address of that location.
//This process is called allocating on the heap.
//Because the pointer to the heap is a known, fixed size, we can store the pointer on the stack, but when we want the actual data, you must follow the pointer. 

/*Pushing to the stack is faster than allocating on the heap because the allocator never has to search for a place to store new data; that location is always 
at the top of the stack. Comparatively, allocating space on the heap requires more work because the allocator must first find a big enough space to hold the data 
and then perform bookkeeping to prepare for the next allocation.*/

//When your code calls a function, the values passed into the function and the function’s local variables get pushed onto the stack. When the function is over, those values get popped off the stack.

//<*-*-*-*-*-*-*-* Ownership Rules *-*-*-*-*-*-*-*-*-*
/*First, let’s take a look at the ownership rules. 
Keep these rules in mind as we work through the examples that illustrate them:

1) Each value in Rust has an owner.
2) There can only be one owner at a time.
3) When the owner goes out of scope, the value will be dropped.*/




//We’ll concentrate on the parts of String that relate to ownership.

/*String literals are convenient, but they aren’t suitable for every situation 
in which we may want to use text. 
One reason is that they’re immutable. 
Another is that not every string value can be known when we write our code: 
for example, what if we want to take user input and store it? For these situations, 
Rust has a second string type, String. This type manages data allocated on the heap and 
as such is able to store an amount of text that is unknown to us at compile time. 
You can create a "String" from a string literal using the "from" function,*/

//In the case of a string literal, we know the contents at compile time, so the text is hardcoded directly into the final executable.
// This is why string literals are fast and efficient. But these properties only come from the string literal’s immutability