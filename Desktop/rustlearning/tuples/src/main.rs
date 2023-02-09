fn main() {
    let some_tuple = (2,3,4, "a", "b".to_string(), 'c', (1.1,2.2));
    println!("1 ::: {} 2::: {} 3 ::: {}", some_tuple.0, some_tuple.1, some_tuple.2);

    println!("{:?}", some_tuple);

    let some_value = some_tuple.6.0;
    println!("{}", some_value);

    let some_clr = get_some_rgb();
    println!("{}", some_clr.0);

    let (red, green, blue) = some_clr;

    println!("red : {}  green: {}   blue: {}", red, green, blue);

    let some_other_color:(u8,u8,u8,u8) = (10,20,30,40);
    let empty_tuple = ();
    match some_other_color.3 {
        50..=100 => println!("value exist in the if statement"),
        _ => ()
    }
}

fn get_some_rgb() -> (u8,u8,u8){
    // some logic

    (100,200,250)
}