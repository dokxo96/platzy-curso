use regex::Regex;
fn main() {
  println!("hola platzy,calcula lo que desees");

  let re_add =  Regex::new(r"(\d+)\s?\+\s?(\d+)").unwrap();
  let re_res =  Regex::new(r"(\d+)\s?\-\s?(\d+)").unwrap();
  let re_div =  Regex::new(r"(\d+)\s?\&\s?(\d+)").unwrap();


  let re_mul =  Regex::new(r"(\d+)\s?\*\s?(\d+)").unwrap();

  println!("ingresa tu calculo");

  let mut expr = String::new();

  std::io::stdin().read_line(&mut expr).unwrap();


  loop{
    let cap=re_mul.captures(expr.as_str());

    if cap.is_none() {
      break;
    }
    let cap= cap.unwrap();

    let cap_expre=cap.get(0).unwrap().as_str();
    let  left_cap:i32=cap.get(1).unwrap().as_str().parse().unwrap();
    let  right_cap:i32=cap.get(2).unwrap().as_str().parse().unwrap();


    let addition =left_cap* right_cap;

    expr=expr.replace(cap_expre,&addition.to_string());

  }
  loop{
    let cap=re_div.captures(expr.as_str());

    if cap.is_none() {
      break;
    }
    let cap= cap.unwrap();

    let cap_expre=cap.get(0).unwrap().as_str();
    let  left_cap:i32=cap.get(1).unwrap().as_str().parse().unwrap();
    let  right_cap:i32=cap.get(2).unwrap().as_str().parse().unwrap();


    let addition =left_cap/ right_cap;

    expr=expr.replace(cap_expre,&addition.to_string());

  }
 loop{
    let cap=re_add.captures(expr.as_str());

    if cap.is_none() {
      break;
    }
    let cap= cap.unwrap();

    let cap_expre=cap.get(0).unwrap().as_str();
    let  left_cap:i32=cap.get(1).unwrap().as_str().parse().unwrap();
    let  right_cap:i32=cap.get(2).unwrap().as_str().parse().unwrap();


    let addition =left_cap+ right_cap;

    expr=expr.replace(cap_expre,&addition.to_string());

  }
  loop{
    let cap=re_res.captures(expr.as_str());

    if cap.is_none() {
      break;
    }
    let cap= cap.unwrap();

    let cap_expre=cap.get(0).unwrap().as_str();
    let  left_cap:i32=cap.get(1).unwrap().as_str().parse().unwrap();
    let  right_cap:i32=cap.get(2).unwrap().as_str().parse().unwrap();


    let addition =left_cap- right_cap;

    expr=expr.replace(cap_expre,&addition.to_string());

  }
 
 

  println!("el resultado es : {} ",expr);
  
}