use std::io;

// fn add_numbers(x: i32, y: i32) -> i32 {
//     // println!("the sum is: {}", x + y);
//     x + y
// }

// fn add_numbers2(x: i32, y: i32) -> i32 {
//     // println!("the sum is: {}", x + y);
//     return x + y;
// }

fn main() {
    println!("Hello world!");

    // prelude
    let mut input = String::new();  // create new empty string
    // get standard input
    io::stdin().read_line(/*reference*/&mut input)
    // catch
    .expect("failed to real line");
    println!("{}", input);


    // let food = "gg";
    // if food == "cookie" {
    //     println!("I like {}", food);
    // }
    // else if food == "gg"  {
    //     println!("I also like {}", food);
    // }
    // else {
    //     println!("I not like {}", food);
    // }

    // add_numbers(23, 39);

    // let number = {
    //     let x = 3;
    //     x + 1
    // };

    // println!("{}", number);

    // let result = add_numbers2(23, 39);
    // println!("{}", result);

    // loops

    // // loop {
    // //     println!("dude again!");
    // // }

    // let mut counter = 0;

    // let result = loop {
    //     counter += 1;
    //     println!("counter: {counter}");

    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };

    // println!("The result is {result}");

    // let mut count = 0;
    // 'counting_up: loop {
    //     println!("count = {count}");
    //     let mut remaining = 10;

    //     loop {
    //         println!("remaining = {remaining}");
    //         if remaining == 9 {
    //             break;
    //         }
    //         if count == 2 {
    //             break 'counting_up;
    //         }
    //         remaining -= 1;
    //     }

    //     count += 1;
    // }
    // println!("End count = {count}");

    // let mut number = 3;

    // while number != 0 {
    //     println!("{number}!");

    //     number -= 1;
    // }

    // println!("LIFTOFF!!!");

    // let a = [10, 20, 30, 40, 50];
    // let mut index = 0;

    // while index < 5 {
    //     println!("the value is: {}", a[index]);

    //     index += 1;
    // }

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

}
