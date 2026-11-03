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

// pub fn main() {
//     let x: i32 = 99;
//     let isEven: bool = evaluator(x);

//     if isEven {
//         println!("x is even")
//     } else {
//         println!("is is odd")
//     }
// }

// pub fn evaluator(x: i32) -> bool {
//     let mut isEven: bool = false;
    
//     if x % 2 == 0 {
//         //set isEvent to true;
//         isEven = true;
//     } else {
//         isEven = false;
//     }
//     return isEven;
// }





// fn main() {
//     // let mut name = String::new();
//     // println!("Enter your name: ");

//     // io::stdin().read_line(&mut name).expect("failed to read input");

//     // println!("Hellow, {name}! Nice to meet you.");

//     let mut input = String::new();



// }



// fn do_something(x: i32) -> i32 {
//     //takes in x as input
//     //does something with x (random number adding to x)
//     //returns the result
//     let random_number_generated: i32 =  rand::random_range(1..=100);
//     let result: i32 = x + random_number_generated;
//     println!("result: {result} and the random number is {random_number_generated}");
//     result
// }

// fn main() {
//     do_something(10);
    
// }



// //declare a few integers on the stack
// fn stack_function() {
//     let a: i32 =10;
//     let b: i32 = 20;
//     let c: i32 = a+b;
//     println!(" the sum of {} and {} is {}", a,b,c);

// }


// //declare things on heap
// fn heap_function() {
//     let s1 = String::from("Hellow");
//     let s2 = String::from("World");
//     let combined = format!("{} {}", s1 , s2);
//     println!("{}", combined);

// }



// fn update_string() {
//     //start with a base string on the heap
//     let mut s = String::from("Initial string");
//     println!("intial string is {}", s);
//     //append some text to this string:

//     s.push_str("and some additional text is added now");
//     //iska output
//     println!("final string is {}", s);
// }



// fn main(){
//     update_string();
//     stack_function();
//     heap_function();
// }




// fn main(){
//     //creating stack variables in a function
//     let x = 1;  //created on stack
//     let y = 3; //created on stack

//     println!("x is {} and y is {} ,and their sum is {}", x, y, sum(x,y));
    
// }


// fn sum(a: i32, b: i32) -> i32 {
//     let c = a+b;
//     return c;
    
// }




// fn main(){
//     let x = 1; //created on stack 
//     {
//         let y = 3; //created on stack

//     }
//     println!("x is {} and y is {}", x , y);
// }


// //passing heap variables as arguments to other functions
// fn main(){
//     let string1 = String::from("Hellow");
//     let string2 = string1;
//     println!("{}", s1)   //this line gives us a compiler error because ownership has been transferred to string2

// }



// fn main(){
//     let string1 = String::from("input string");

//     takes_ownership(string1);
//     println!("{}", string1);    //gives compiler error because ownership has been transferred to some_other_string
// }

// fn takes_ownership(some_other_string: String) {
//     println!("input string is {}", some_other_string);
// }



// //fix: 
// fn main(){
//     let string1= String::from("this is string 1");
//     let string2 = string1.clone();
//     println!("string 1 is {}", string1);
//     println!("string 2 is {}", string2);
// }


//referencing , the main ownership still lies with s1. but s2 can atleast point and read s1
// fn main(){
//     let s1 = String::from("hellow");
//     let s2 = &s1;
    
//     println!("s1 is {}", s1);
//     println!("s2 is {}", s2)
// }




//borrowing -- you can transfer ownership of variables to functions. 
//by passing a reference to the string to the function take_ownership
//the ownership of the string remains with the original variable , in the main function
//this allows you to use my_string again after the function call




// fn main(){
//     let my_string = String::from("hellow, rust");
//     takes_ownership(&mystring);
//     println!("{}", my_string);
// }


// fn takes_ownership(input_string: &String) {
//     println!("{}", input_string);
// }



// fn main(){
// 	let string1 = String::from("hellow, amaan");
// 	print_name(string1);
	
// 	println!("{}", string1)   //here string 1 will give compile error because ownership has transferred to the print_name function
	
	
// }



// fn print_name(input: String) {
// 	println!("{}", input);
// }


//mutable borrwo
// fn main(){
//     let mut name = String::from("Shahrukh");
//     change_name(&mut name);

//     println!("name is {}", name);
// }


// fn change_name(inputString: &mut String) {
//     inputString.push_str(" Khan");
// }




// struct User {
//     isActive: bool,
//     name: String,
//     email: String,
//     age: i32,
//     sign_in_count: u64,
// }
// #[derive(Debug)]
// fn main() {
//     let user1 = User {
//         isActive: true,
//         name: String::from("Amaan"),
//         email: String::from("amaan@gmail.com"),
//         age: 20,
//         sign_in_count: 1,
//     };
//     println!("user 1 is {:?}", user1);
// }

// #[derive(Debug)]
// struct User {
//     is_active: bool,
//     name: String,
//     email: String,
//     age: i32,
//     sign_in_count: u64,
// }

// fn main() {
//     let user1 = User {
//         is_active: true,
//         name: String::from("Amaan"),
//         email: String::from("amaan@gmail.com"),
//         age: 20,
//         sign_in_count: 1,
//     };
//     println!("user 1 is {:?}", user1);
//     println!("user 1 is {:#?}", user1);
// }



//only stack types in struct:
// struct User {
//     active: bool, 
//     sign_in_count: u64,
// }

// fn main(){
//     let mut user1 = User {
//         active: true, 
//         sign_in_count : 1,
//     };

//     print_name(user1);
//     println!("User 1 username: {}", user1.active)
// }


// struct User{
//     active: bool , 
//     sign_in_count : u64,
// }

// fn main(){
//     let mut user1 = User {
//         active: true, 
//         sign_in_count : 1,
//     };
    
//     print_name(user1);
//     println!("user 1 is {:?}", user1); //error -- can not use borrowed values
// }


// fn print_name(user1: User){
//     println!("user 1 username : {}", user1.active)
// }



// #[derive(Copy, Clone)]
// struct User {
//     active: bool, 
//     sign_in_count : u64,
// }


// fn main(){
//     let mut user1 = User {
//         active: true, 
//         sign_in_count : 1,
//     };

//     print_name(user1);
//     println!("User 1 username: {}", user1.active)  //error goes away because user 1 is copied

// }


// fn print_name(user1: User){
//     println!("User 1 username : {}", user1.active)
// }



//---------------------------------------//


// struct User {
//     active : bool,
//     username: String, 
//     email: String , 
//     age : i128,
// }

// fn main(){
//     let user1 = User {
//         active: true,
//         username: String::from("Amaan"),
//         email: String::from("amaan@gmail.com"),
//         age: 20,
//     };
//     println!("user 1 is {:?}", user1);
// }



// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

// fn main() {
//     let user1 = User {
//         active: true,
//         username: String::from("someusername123"),
//         email: String::from("someone@example.com"),
//         sign_in_count: 1,
//     };
//     println!("User 1 username: {:?}", user1.username);
// }



// #[derive(Debug)]
// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

// fn main() {
//     let user1 = User {
//         active: true,
//         username: String::from("someusername123"),
//             email: String::from("someone@example.com"),
//         sign_in_count: 1,
//     };
//     println!("User 1 username: {:?}", user1);
// }   


// struct User {
//     active: bool,
//     name: String, 
//     email: String,
// }


// fn main(){
//     let mut originalUser = User {
//         active: true,
//         name: String::from("Amaan"),
//         email: String::from("amaan@gmail.com"),
//     };
//     println!("original user before naamikaran, {}", originalUser.name);
//     change_name(&mut originalUser);
//     println!("original user after naamikaran, {}", originalUser.name)
// }



// fn change_name(input : &mut User) {
//     //changes the name to ayaan , every name btw
//     input.name = String::from("Ayaan");
// }




#[derive(Clone)]
struct User {
    active : bool,
    sign_in_count : u64,
    username: String,
}


fn main(){
    //define user 1 
    let mut user1 = User {
        active: true, 
        sign_in_count : 1,
        username: "Amaan".to_string(),
    };
    print_name(user1.clone());   //NOTE: IF YOU WRITE THE CODE W THIS LINE, GIVES ERROR BECAUSE OWNERSHIP GOES TO OUR ANOTHER FUNCTION. IF YOU WANT THE BELOW LINES TO PRINT , THEN YOU CANNOT TRANSFER OWNERSHIP, RATHER YOU JUST USE CLONE 
    println!("user 1 ka status is {}", user1.active)

}


fn print_name(inputString : User) {
    println!("user 1 ka username is {} ", inputString.username)
}