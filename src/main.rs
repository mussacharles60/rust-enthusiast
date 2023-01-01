// fn add_numbers(x: i32, y: i32) -> i32 {
//     // println!("the sum is: {}", x + y);
//     x + y
// }

fn add_numbers2(x: i32, y: i32) -> i32 {
    // println!("the sum is: {}", x + y);
    return x + y;
}

fn main() {
    println!("Hello world!");
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

    let result = add_numbers2(23, 39);
    println!("{}", result);
}
