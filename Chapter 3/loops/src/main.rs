fn main() {
    //loop {
        //println!("again!");
   // } //Infinte loops
    //We can use "break" to stop the loop and "continue" to 
    //skip over any remaining code in this iterationa nd go to the nest iteration.

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");


 //<---------------Loop Labels to Disambiguate Between Multiple Loops---------------->
    
    let mut count = 0;
        'counting_up: loop { //This is a label 'counting_up'
            println!("count = {count}");
                let mut remaining = 10;

            loop {
                println!("remaining = {remaining}");
                if remaining == 9 {
                    break;
                }   
                if count == 2 {
                    break 'counting_up;
                }
                remaining -= 1;
            }

            count += 1;
        }
    println!("End count = {count}");
  
//<---------Conditional Loops with while-------------->

    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
        if number == 1{
            break;
        }
    }

    println!("LIFTOFF!!!");

//<----------------Looping through a collection with for----------->
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    /* This approach is error prone; we could cause the program to panic if the 
    index value or test condition is incorrect. For example, if we changed the definition of the a array 
    to have four elements but forgot to update the condition to while index < 4, the code would panic.*/
    let a1 = [10, 20, 30, 40, 50];

    for element in a1 {
        println!("the value is: {element}");
    }
    /* We’ve now increased the safety of the code and eliminated the chance of bugs 
    that might result from going beyond the end of the array or not going far enough 
    and missing some items.*/


    for number in (1..4).rev() {  //rev, to reverse the range
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
