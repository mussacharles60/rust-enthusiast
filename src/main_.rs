use std::io;

fn main() {
    println!("rust-2 start");

    // let x = 4;
    // // let x: u32 = 4;
    // println!("x is: {}", x);

    // { // independent scope
    //     let x = " hello";
    //     println!("x is: {}", x);
    // }

    // let x = x + 5;
    // println!("x is: {}", x);

    // const VALUE_IN_CAPITAL: u32 = 50;
    // println!("{}", VALUE_IN_CAPITAL);

    // scalar: single value
    // compound: multiple value

    // signed integers
    // integers only without decimal point. eg: -1 12, 456, 5999
    // i8   0 ---- 2^8-1 0 - 255
    // i16
    // i32
    // i64
    // i128

    // unsigned integers
    // integers only without decimal point. eg: -1 12, 456, 5999
    // u8 -2^7 ---- 2^7-1
    // u16
    // u32
    // u64
    // u128

    // float
    // f32 bit (single precision)
    // f64 bit (double precision)

    // // eg:
    // let my_float = 10.9;
    // let floating_point: f32 = 10.9;

    // // boolean
    // let is_yes = true;
    // let is_ok: bool = false;
    // let is_also: bool = 1;
    // let is_yesss: bool = 0;

    // // character char (single character)
    // let letter = 'a';
    // let letter1: char = 'a';

    // tup po

    // let tup = (1, true, 's');
    // println!("{}", tup.1);

    // let mut tup = (1, true, 's');
    // tup.0 = 10;
    // println!("{}", tup.0);

    // arrays
    // let mut arr = [1, 2, 3, 4, 5];
    // arr[4] = 56;
    // println!("{}", arr[4]);

    // let mut arr = [];
    // arr[4] = 56;
    // println!("{}", arr[4]);

    // prelude
    // let mut input = String::new();  // create new empty string
    // // get standard input
    // io::stdin().read_line(/*reference*/&mut input)
    // // catch
    // .expect("failed to real line");
    // println!("{}", input);

    // arthmetic

    // let x: u8 = 9; // 0 - 255
    // let y: i8 = 10; // -128 - 127

    // let x: f32 = 9_000.0;
    // let y = 10_000.0_f32;
    // let p = 10_000.0 as f32;

    // let z = x + y + p;
    // println!("{}", z);

    // let mut input = String::new(); // create new empty string
    //                                // get standard input
    // io::stdin()
    //     .read_line(/*reference*/ &mut input)
    //     // catch
    //     .expect("failed to real line");
    // println!("{}", input);

    // let int_input: i64 = input.trim().parse().unwrap();
    // println!("{}", int_input + 3);

    //
}
