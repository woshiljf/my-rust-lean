
mod front_of_house;

fn main() {
  
    // let v: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3];
    v.push(7);
    let third: &i32 = &v[2];
    println!("第三个元素是 {}", third);
  
    match v.get(2) {
      Some(third) => println!("第三个元素是 {}", third),
      None => println!("没有第三个元素"),
    }

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }
    

}
