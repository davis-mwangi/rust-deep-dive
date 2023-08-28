/**
OWNERSHIP: set of rules that govern memory management
           Rules are enforced at compile time
           If any of the rules are violated the program wont compile

Three Rules of Ownership
1) Each value in Rust has an owner
2) There can only be one owner at a time
3) When the owner goes out of scope, the value will be dropped

General Rule: scope ends when the curly braces ends


Stack Memory: stores data of known size

Heap Memory:
   - Stores data of no known , fixed size
   - Allocating data on heap memory will return a pointer(
     an address to location where data has been allocated)
   - Allocating on the heap is slower than pushing to stack
   - Accessing data on the heap is also slower , as it has to be
     accessed using a pointer which points to an address.
*/
fn main(){
    let x = 42;
    let y =  10;
    let z =  add_numbers(x, y);

    println!("the result is {}", z);

    //COPY vs MOVE
    //copy
    let x: i32 = 5;
    let y : i32 = x;
    let z = x + y;
    println!("{}", z);
    //Move: s1 will be dropped and s2 will take ownership
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}", s1 + &s2);

}
fn add_numbers(a: i32,  b: i32) -> i32{
    let c =  a +  b;
    c
}