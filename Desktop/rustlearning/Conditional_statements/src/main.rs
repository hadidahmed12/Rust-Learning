fn main() {
    let some_bool = false;
    let some_int = 50;
    let some_int2 = 2;

    if some_bool == true || some_int > 100 && some_int2 >= 100 {
        println!("Hi i'm in the true statement! IF_Condition");
    } 
    else if some_int == 50 {
        println!("I'm in Else IF Statement");
    } 
    else {
        println!("Hi i'm in the faslse statement");
    }

    //inline conditional statement
    let value_from_inline = if some_int == 40 {500}
    else if some_int2 == 100 {
        println!("Hi i'm in 400");
        400
    }
    else {200};

    println!("INline Statement value:: {}", value_from_inline);


    match some_bool{
        true => {println!("Hi i'm in the true statement");}

        false =>{ println!("Hi False");}
    }

    match some_int {
        0 => println!("Hit 0 number ::"),
        1..=100 => println!("Hi 1...100 number::"),
        _ => 
            println!("Else statement"),
    }

    // inline match statement 

    let value_from_match = match some_int2 {
        0 => {println!("Hi i'm in the IF statement");}
        1|2 => {println!("i,m in else if 1|2");} //some int is 1 OR 2 this means
        _ => { println!("");}
    };
    println!("value of match is :: {:?}", value_from_match );
}
