
pub trait SomeTrait {
    fn is_valid(&self) -> i64;
}

impl SomeTrait for Randominfo {
    fn is_valid(&self) -> i64 {
        self.some_int
    }
}

#[derive(Debug)]

pub struct Randominfo{
   pub call_count: i64,
   pub some_bool: bool,
   pub some_int: i64,
}

impl Randominfo { 
    pub fn new(param_a:bool) -> Self {
         Self{
            call_count: 0,
            some_bool: !param_a,
            some_int: 8,
         }
    }

    pub fn is_smaller(& mut self, compare_to:i64) -> bool {
        self.call_count += 1;
        self.some_int < compare_to
      
    }
    // pub fn is_smaller(& mut self, compare_to:i64) -> i64 {
    //     self.some_int < compare_to;
    //     self.call_count += 1;
    //     self.call_count
      
    // }
}

/////////// More Type of Structs /////////////////////

//struct without body////////
// struct nobodystruct{}


///////Traits Struct//////
// struct  pair<T> {
//     x: T, y: T,
// }

//////tuple struct///////
// struct color(u8,u8,u8);