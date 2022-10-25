fn main() {
    let mut name:String=String::new();
    let mut age: String=String::new();;

    println!("Hello, world! wirte your name:");
    std::io::stdin().read_line(&mut name).unwrap();
    println!("good,now your age:");
    std::io::stdin().read_line(&mut age).unwrap();

    let n_Age:u8=age.trim().parse().unwrap_or(0);
    println!("your name is {} and your age is :{}", name,n_Age);
}
