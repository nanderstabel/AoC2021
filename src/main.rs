







fn func(mut c: String) -> String {

    println!("func: {:?}", c);
    c.push_str(" world!");
    c
}


fn func_ref(c: &mut String) {

    println!("func_ref: {:?}", c);
    c.push_str(" world!");
}




fn main() {
    let mut a = String::from("hello");

    println!("main: {:?}, {:p}", a, &a);
    a = func(a);
    println!("main: {:?}, {:p}", a, &a);
    func_ref(&mut a);
    println!("main: {:?}, {:p}", a, &a);
    // println!("{:?}", b);

}
