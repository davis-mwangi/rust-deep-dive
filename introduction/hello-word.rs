/**
Numbers:
  Integers: i32
  Floats: f64


**/
use std::ops::{Range, RangeInlusive};

fn main() {

    let _x: i32;
    let _y: i32;

    //Mutation
    let mut x = 1;
    x += 2;

    println!("{}",x);

    println!("Hello, World");

    define_x();


    //Overshadowing
    let mut z : i32 = 5;
    let mut z:i32 = 7;

    //Destructing
    let (x, y) = (1,2);
    x += 2;
    assert_eq!(x, 3);
    assert_eq!(y, 2);
    //Numbers

    //Floating Poiny
    let mut x: f64 = 1_000.000_1;
    asser!(0.1 as f32  + 0.2  as f32  == 0.3 as f32)//0.1 +  0.2 =  0.30000000000005


    //Range
    let mut sum: i32 = 0;
    for i in -3..2 {
        sum  +=  i;
    }
    assert!(sum == -5);
    for c in 'a'..='z' {// = indicate z is included
        println!("{}", c);
    }
    assert_eq!((1,5), Range{start: 1, end: 5} );
    assert_eq!((1..=5), RangeInclusive::new(1,5));

    //Pattern Matching(switch)
    fn get_option(tp: u8) -> Option<i32> {
        match tp {
            1 => {
                //Do something when tp matches 1
            }
            _ => {
                //Match anything else
            }
        }
        never_return_function();
    }
    fn never_return_function(){
        panic!();//This will cause the function to never return and exit
    }


}
//Scope
fn define_x(){
    let x: &str ="Hello";
    println!("{}, World-2", x);
}