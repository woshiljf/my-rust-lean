

fn main() {

    // 更新string
    let mut s = String::from("foo");

    s.push_str("bar");

    println!("{}", s);


    // 使用push 方法更新string
    let mut  s1 = String::from("hello");
    s1.push('3');




    let s2 = String::from("Hello, ");
    let s3 = String::from("world! ");
    // s1 所有权被转移了
    let s4 = s2 + &s3; // 注意 s1 被移动了，不能再使用

   
   print!("{}", s4);
//    print!("{}", s2);
   print!("{}", s3);


   // format 方法拼接

   let s1 = String::from("tic");
   let s2 = String::from("tac");
   let s3 = String::from("toe");

   let s = format!("{}-{}-{}", s1, s2, s3);

   println!("{}", s);

}
