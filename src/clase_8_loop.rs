fn main() {
    let mut n1=123;
    let mut n2=321;
    let suma= n1+n2;


    loop {
        println!("cual es la suma de {} + {}", n1, n2);

        let mut sume_user=String::new();

        std::io::stdin().read_line(&mut sume_user).unwrap();

        let suma_user_res:i16 =sume_user.trim().parse().unwrap();
       
        if suma_user_res== suma{
            println!("el resultado es correcto");
            println!("fin.....");
            break;
        }else{
            println!("el resultado no es correcto");
            println!("intetalo de nuevo............");
        }
    }
    

  
}
