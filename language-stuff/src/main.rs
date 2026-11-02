// fn main() {
//     println!("this is the language stuff folder");
//     //define variables :
//     let _x = 1;
//     println!("x is {_x}");

//     let y: i32 = 2;
//     println!("y is {y}")

// }

//need the _ if you are ignoring data type
//

// fn main() {
//     let mut num: i8 = 124;
//     for  _i in 0..100 {
//         num += 127;
//     }
//     print!("Number: {}", num)
// }

// fn main() {
//     let isMale: bool = false;
//     let isAdult: bool = true;

//     //if male and adult, then only vote
//     if isMale && isAdult {
//         println!("eligible to vote")
//     }
//     else if !isAdult{
//         println!("not eligible to vote")
//     }
//     else if isMale {
//         println!("eligible to vote but not an adult")
//     }
//     else {
//         println!("{}", isMale);
//         println!("{}", isAdult);
//         println!("not eligible to vote and not an adult")
//     }

// }

pub fn main() {
    let x: i32 = 99;
    let isEven: bool = evaluator(x);

    if isEven {
        println!("x is even")
    } else {
        println!("is is odd")
    }
}

pub fn evaluator(x: i32) -> bool {
    let isEven: bool = false;
    
    if x % 2 == 0 {
        //set isEvent to true;
        isEven = true;
    } else {
        isEven = false;
    }
    return isEven;
}
