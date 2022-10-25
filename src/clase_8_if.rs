fn main() {
    let mut name:String=String::new();
    let mut age: String=String::new();;

    println!("Hello, world! wirte your name:");
    std::io::stdin().read_line(&mut name).unwrap();
    println!("good,now your age:");
    std::io::stdin().read_line(&mut age).unwrap();

    let n_Age:u8=age.trim().parse().unwrap_or(0);

    if n_Age>= 18 {
        println!("your name is {} and your age is :{},so you can get into the club,enjoy it", name,n_Age);
    }
    else{
        println!("your name is {} and your age is :{},so you cannot get into the club,get to home boy", name,n_Age);

    }
  
}
