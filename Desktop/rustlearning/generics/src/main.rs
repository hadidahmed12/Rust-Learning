// #[derive(Debug)]
//////////These are the explicit type struct which is defined by the programmer itself//////////////////
// struct Point {
//     x: i32,
//     y: i32,
// }
// struct Point{
//     x:f64,
//     y:f64,
// }

/////////////Now we use generics struct to generic the struct to reduct the code and duplication///////////////////
/// /////// There is no run time cost for using generics all done at compile time///////////////// 

// struct Point<T> {
//     x: T,
//     y: T,
// }

//// If you want to use generics struct for two different datatypes ///////

// struct Point <T, U>{
//     x: T,
//     y: U,
// }

//******************** GENERICS ENUMS *************************/
// enum SomeEnum<T>{
//     OptionA(T),
//     OptionB(T),
//     OptionC(T),
// }


fn main() {

    // we use "POINT struct in different datatypes Example float, int , String"
//    let a = Point{x: 100, y: 200.10};
//    println!("{:?}", a);
//    let b = Point {x:200, y: 500.52};
//    println!("\n{:?}", b);
//    let c = Point {x: "Hadid".to_string(), y: true};
//    println!("\n{:?}", c);

//*********Enums *********** */
    //   let somedata = SomeEnum::OptionA("Hadid Ahmed".to_string());
    //   println!("{:?}",somedata); 
    //   some_process(somedata);
    //   let somedata1 = SomeEnum::OptionB(500.50);
    //   println!("{:?}", somedata1);
    //   some_process(somedata1);
    //   let somedata2 = SomeEnum::OptionC;
    //  println!("{:?}", somedata2);
     // some_process(somedata2);

     //**********GENERICS FUNCTIONS***********/
     let i1 = dougs_func2(100.5,50.8,30);

     println!("Sumof Input1 & input2 is {:?} & Input3 sum is {:?}", i1);
 
}

// fn some_process<T:std::fmt::Display + std::fmt::Debug> (somedata: SomeEnum<T>) {
    // match somedata2 {
    //     SomeEnum::OptionA(a)=> {
    //         println!("You are in the option A :: {} ",a);
    //     } 
    //     SomeEnum::OptionB(b)=> {
    //         println!("You are in the option B ::{} ",b);
    //     }
    //     SomeEnum::OptionC=> {
    //         println!("You are in the option C which is very boring:: ");
    //     }
    // }
   
// }

// use core::ops::Sub;
//  use std::ops::Sub<Output = T>;
// fn dougs_func<T:std::ops::Sub<Output = T> + std::fmt::Display + std::fmt::Debug> (input1 : T, input2: T) -> T{
//     println!("\nInput1 is {}",input1);
//     println!("Input2 is {:?} \n",input2);
//     let sum = input1 - input2;
//     sum
// }
#[allow(dead_code)]
fn dougs_func2<T, E> (input1 : T, input2: T, input3: E) -> (T , E)
    where T:std::ops::Add<Output = T> + std::ops::Sub<Output = T> + std::fmt::Display + std::fmt::Debug,
    E: std::fmt::Debug {
    println!("\nInput1 is {:?}",input1);
    println!("Input2 is {:?} \n",input2);
    println!("Input3 is {:?} \n",input3);

    let mut sub = input1 - input2 ;
    // sub = sub + input3;
    (sub, input3)
}