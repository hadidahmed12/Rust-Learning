/*****************Functions and procedures************************ */

// functions return a value but procedure do not.....

fn main(){
let get_function = some_function(105.2, 56, 22);
println!(" Return data {}", get_function);

some_procedure(80.6, 108);
some_str_procedure("HADID AHMED");
let string_str: &str = "Some thing has done by me";

some_str_procedure(string_str);

let real_string = String::from("I'm a real string");
some_str_procedure(&real_string);
some_string_procedure(real_string);
// some_str_procedure(&real_string);// value borrowed here after move
}



#[allow(dead_code)]
// This is procedure & procedure do not return a value

fn some_procedure(param_a : f32, param_b : i128){
    println!("{} {}", param_a, param_b);
}
// This is Function example
fn some_function (param_a: f64, param_b: i128, _param_c: i32)  -> f64 {

    // 10.5
    // 10 as f32
    
     if param_a < 100. {
        let return_value = 10.5  * param_a  + param_b as f64;
        println!("Hello Hadid \n you got {} marks",return_value);

        return_value // no  semicolon use in return statment....   
     }
     else {
        -1.
     }
}   
fn some_str_procedure(param_a:&str){

    println!("I'm in some procedure :: Name : {}", param_a);

}

fn some_string_procedure(param_a:String){
    println!("I'm in some string :: Name : {}", param_a);
}