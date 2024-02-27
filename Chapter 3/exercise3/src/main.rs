// //Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.

// fn main() {
//     let mut day = 1;
//     let mut count = 1;
//     while day <= 12 {
//         println!("On the {day} day of Christmas,");
//         println!("my true love gave to me");
//         if day == 1{
//             println!("A partridge in a pear tree.");
//         }else {
//             //println!("{day} turyle doves,");
//             //println!("And a partridge in a pear tree.");
//             for number in (2..day).rev() {  //rev, to reverse the range
//                 println!("{day} turyle doves,");
//             }
//             println!("And a partridge in a pear tree.");
//         }
//         day += 1;
//     }
// }

fn main() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth",
        "eleventh", "twelfth"
    ];

    let gifts = [
        "a partridge in a pear tree",
        "Two turtle doves,",
        "Three French hens,",
        "Four calling birds,",
        "Five golden rings,",
        "Six geese a-laying,",
        "Seven swans a-swimming,",
        "Eight maids a-milking,",
        "Nine ladies dancing,",
        "Ten lords a-leaping,",
        "Eleven pipers piping,",
        "Twelve drummers drumming,"
    ];

    for day in 0..12 {
        println!("On the {} day of Christmas,", days[day]);
        println!("My true love sent to me",);
        if day == 0{
            println!("A partridge in a pear tree.");
        }
        for gift in (0..=day).rev() {
            if gift == 0 && day != 0 {
                println!("And {}", gifts[gift]);
            } else if gift != 0 && day != 0{
                println!("{}", gifts[gift]);
            }
        }

        println!();
    }
}
