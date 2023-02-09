mod random_info;
use random_info::*;

#[allow(dead_code)]
#[derive(Debug)]
struct DougsData {
    some_bool: bool,
    some_float: f64,
    some_int: i64,
    random: Randominfo,
}

impl Randominfo {
    pub fn is_larger(&self,compare_to: i64) -> bool {
        self.some_int > compare_to
    }
}

// impl SomeTrait for DougsData {
//     fn is_valid(&self) -> i64 {
//         self.some_int
//     }
// }

// fn print_if_is_valid(checkme: &dyn SomeTrait , checkme2: &dyn SomeTrait){
//  if checkme.is_valid() == checkme2.is_valid(){
//     println!("yayyy bruu");
//  }
//  else {
//     println!("noooo bruuu");
//  }
// }

impl Default for DougsData {
    fn default() -> Self {
        Self { some_bool: true, some_float: 50.2, some_int: 10 , random: Randominfo::new(true),}
    }
}

#[allow(unused_variables)]


fn main() {

    let dougsvar0 = Randominfo{
        call_count: 0,
        some_bool: true,
        some_int: 10,
    };
    println!("your Random info data is :: {:#?}", dougsvar0);

    // let is_this_smaller = dougsvar0.is_smaller(15);
    let is_this_larger = dougsvar0.is_larger(4);

    let is_valid = dougsvar0.is_valid();

    println!("\n \n your Random info data Bool is :: {}", is_valid);

    // println!("\n \n your Random info data is SMALLER :: {}", is_this_smaller);
    println!("\n \n your Random info data is SMALLER count :: {}", dougsvar0.call_count);

    println!("\n \n your Random info data is larger :: {}", is_this_larger);


    println!("\n \n your Random info data is :: {:#?}", dougsvar0);

    
        let dougsvar = DougsData::default();

        println!("DougsData  Structure :: {:?}", dougsvar);
 
    // let dougsvar = DougsData{
    //     some_bool: true,
    //     some_float: 10.5,
    //     some_int: 10,
    //     random: Randominfo::new(true),
    //     // random: Randominfo{
    //     //     some_bool: false,
    //     //     some_int: 120,
    //     // }
    // };

    //  println!("polymorfism :: {:#?}", dougsvar.random);

    //  dougsvar.some_bool = false;
    //  println!("{:#?}", dougsvar);

    //  let dougsvar2 = DougsData {
    //     some_int: 150,
    //     some_float: 50.6,
    //     some_bool: true,
    //     ..dougsvar
    //  };
    //  println!("{:#?}", dougsvar2);

    // print_if_is_valid(&dougsvar0, &dougsvar);
    // print_if_is_valid(&dougsvar);
}
