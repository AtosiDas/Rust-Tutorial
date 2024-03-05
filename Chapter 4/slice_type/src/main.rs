//Slices let us reference a contiguous sequence of elements in a collection rather than the whole collection. A slice is a kind of reference, so it does not have ownership.

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();  //as_bytes() converts convert our String to an array of bytes""
    //"iter" is a method that returns each element in a collection and "enumerate" wraps the result of "iter" and returns each element as part of a tuple instead.
    //The first element of the tuple returned from enumerate is the index, and the second element is a reference to the element.
    for (i, &item) in bytes.iter().enumerate() {  //we create an iterator over the array of bytes using the iter method
        if item == b' ' {  //Inside the for loop, we search for the byte that represents the space by using the byte literal syntax. 
            return &s[0..i];
        }
    }

    &s[..]
}
fn main() {
    let mut s = String::from("Hello World");
    let word = first_word(&s); //word will get the value 5
    println!("{}",s);
    println!("{}",word);
    // s.clear(); // this empties the string, making it equal to ""
    // println!("{}",s);
    // println!("{}",word); // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word1 = first_word(&my_string[0..6]);
    let word2 = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word3 = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word11 = first_word(&my_string_literal[0..6]);
    let word12 = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word13 = first_word(my_string_literal);
    println!("{}", word1);
    println!("{}", word2);
    println!("{}", word3);
    println!("{}", word11);
    println!("{}", word12);
    println!("{}", word13);
}

// fn second_word(s: &String) -> &str {
//     //let s = String::from("hello world");
//     //A string slice is a reference to part of a String
//     let hello = &s[0..5];  // We can also write this "hello = &s[..5]"
//     let world = &s[6..]; // We can also write this "world = &s[6..]"

//     //To take an entire string as a slice, we write 
//     //word = &s[..]
// }

//<*-*-*-*-*--*---* String Slices *-*-*-**-*-**-*-*-*-*

/* A string slice is a reference to part of a String
We create slices using a range within brackets by specifying [starting_index..ending_index], 
where starting_index is the first position in the slice and ending_index is one more than the last position in the slice. 
Internally, the slice data structure stores the starting position and the length of the slice, which corresponds to ending_index minus starting_index*/

/*Recall from the borrowing rules that if we have an immutable reference to something, we cannot also take a mutable reference.
Because clear needs to truncate the String, it needs to get a mutable reference.*/

//<--------------Important---------------->
/*Rust disallows the mutable reference in clear and the immutable reference in word from existing at the same time, and compilation fails.*/

//<************* String Literals*************
/*we talked about string literals being stored inside the binary. Now we know about slices, we can properly understand string literals:

let s = "Hello, world!";
The type of s here is &str: it’s a slice pointing to that specific point of the binary. 
This is also why string literals are immutable; &str is an immutable reference.*/


//*****************String Slices as parameters********************
//If we have a string slice, we can pass that directly. If we have a String, we can pass a slice of the String or a reference to the String.


//<-------------------------Other Slices------------->
/*String slices, as you might imagine, are specific to strings. But there’s a more general slice type too. Consider this array:

let a = [1, 2, 3, 4, 5];
Just as we might want to refer to part of a string, we might want to refer to part of an array. We’d do so like this:

let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];

assert_eq!(slice, &[2, 3]);
This slice has the type &[i32]. It works the same way as string slices do, 
by storing a reference to the first element and a length. We’ll use this kind of slice for all sorts of other collections. */