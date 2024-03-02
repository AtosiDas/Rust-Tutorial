fn main() {  
    println!("Hello, world!"); // s is not valid here, it's not yet declared
    //A scope is the range within a program for which an item is valid.
    //The variable s refers to a string literal. The variable is valid from the point at which it’s declared until the end of the current scope.
    let s ="Atosi";  // s is valid from this point forward
    // do stuff with s

    let mut s1 = String::from("Welcome"); //The double colon :: operator allows us to namespace this particular from function under the String type rather than using some sort of name like string_from.
    //println!("{}", s1);
    println!("{s1}");
    s1.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s1); // This will print `Hello, world!`
}  // this scope is now over, and s is no longer valid.





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

