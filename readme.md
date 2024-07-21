# 持续学习rust


## 第八章-数据结构

### 8.1 Vector

1. Vector 的创建

```

  let v: Vec<i32> = Vec::new();

  let v = vec![1, 2, 3];
  // push 值
  v.push(7);

```

2. 访问元素

```

  let v = vec![1, 2, 3, 4, 5];

  let third: &i32 = &v[2];
  println!("第三个元素是 {}", third);

  match v.get(2) {
    Some(third) => println!("第三个元素是 {}", third),
  }

```

3. 所有权和借用规则

- 不能在同一时间拥有一个可变的引用和一个不可变的引用

```

  let mut v = vec![1, 2, 3, 4, 5];
   // 声明了不可变的变量first
  let first = &v[0];
  v.push(6);
  println!("第一个元素是: {}", first);

```

4. 迭代

```

  let v = vec![100, 32, 57];
  for i in &v {
      println!("{}", i);
  }

```

5. 使用枚举来存储多种类型的值

```

  enum SpreadsheetCell {
      Int(i32),
      Float(f64),
      Text(String),
  }

  let row = vec![
      SpreadsheetCell::Int(3),
      SpreadsheetCell::Text(String::from("blue")),
      SpreadsheetCell::Float(10.12)]

```

### 8.2 String
 
1. 创建String

```

   // 创建了一个String 类型
    let mut s = String::new();

    let data = "initial contents";
    
    // 转化为String 类型
    let s = data.to_string();
    
    // 使用from 方法
    let s = String::from("initial contents");

```

2. 更新String
- push_str 方法

```

    let mut s = String::from("foo");
    s.push_str("bar");

```
- push 方法
单个字符push

```

    let mut s = String::from("lo");
    s.push('l');

```

3. 字符串拼接

'+' :连接字符串


```

    let s1 = String::from("Hello, ");
    let s2 = String::from("world! ");
    // s1 所有权被转移了
    let s3 = s1 + &s2; // 注意 s1 被移动了，不能再使用

```

- format 方法

```

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);

```