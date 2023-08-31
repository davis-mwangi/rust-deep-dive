fn main(){
    let s  =  String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
    //&str (string slice)- immutable

    //String- mutable
    let mut s : String = String::from("");
    s.push_str("hello, world");
    s.push('!');
    s += "!";

    assert_eq!(s, "hello, world!!");
    println!("Success");

    //Replace substring
    let s: String = String::from("I like dogs");

    //Allocate new memory and store the modified string there
    let s1 =  s.replace("dogs", "cats");

    assert_eq!(s1, "I like cats");

    println!("Success!");

    //Concat String
    let s1 : String =  String::from("hello,");
    let s2 : String  =  String::from("world");
    let s3: String = s1 + s2.as_str();//String-> &str
    assert_eq!(s3, "hello,world");
    println!("{}", s3);

    //&str -> String
    let s: &str = "hello, world";
    greetings(s.to_string());// &str - > String // or String::from(s)

    let s = "hello, world".to_string();
    let s1: &str = &s;// &String -> &str

    println!("Success!");


    //STRING INDEX
    let s1 :String = String::from("h1, %##$@");
    let h1:&str = &s1[0..1];//Get the first character
    assert_eq!(h1,"h");
    println!("Success!");

    for c in "果 红 下 山".chars() {
         println!("{}",c);
    }
}

fn greetings(s: String){
    println!("{}",s);
}