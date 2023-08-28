/**
Way of temporariy access data without taking ownership of it
When borrowing you are taking  a `reference(pointer)` to the data, not the data itself
Prevention of dangling pointers and data races
Data can borrowed `immutabily` and `mutably`

Rules of References
1) At any given time, you can have either `one mutable reference` or `any number` of `immutable references`
2) References must be `always be  valid`
**/
fn main(){
    let s1 =  String::from("hello");
    let len  = calculate_length(&s1);
    println!("The length of '{}' is {}", s1, len);

    let mut s = String::from("hello");
    change(&mut s);

}
fn calculate_length(s: &String) -> usize {
    s.len()
}
fn change(some_string:  &mut String){
    some_string.push_str(", world");
}