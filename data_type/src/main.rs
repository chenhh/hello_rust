fn main() {
    // 使用 parse 將 String 轉換為數字時，必須增加類型註解
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("guess: {}", guess);

    // 整數, 預設為i32
    let x1 = 98_222;
    let x2 = 0xff;
    let x3 = 0o77;
    let x4 = 0b1111_0000;
    let x5: u8 = b'A';
    println!("x1:{}", x1); // 98222
    println!("x2:{}", x2); // 255
    println!("x3:{}", x3); // 63
    println!("x4:{}", x4); // 240
    println!("x5:{}", x5); // 65

    // 浮點數, 預設為f64
    let f1 = 2.0;
    let f2: f32 = 3.0;
    println!("f1:{}", f1); // 2.0
    println!("f2:{}", f2); // 3.0

    // 運算
    println!("sum:{}", 5 + 10); // 15
    println!("prod:{}", 5 * 10); // 50
    println!("quotient: {}", 56.7 / 32.2);
    println!("floored 2/3: {}", 2 / 3); // 0
    println!("reminder: {}", 43 % 5); //3

    // boolean type
    let t = true;
    let f: bool = false; // 顯式指定類型註解
    println!("t: {}", t);   // true
    println!("f: {}", f);   // false

    // char, Unicode (4 bytes)
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("c:{}, z:{}, h:{}", c, z, heart_eyed_cat);

    /* tuple
     * 元組長度固定：一旦聲明，其長度不會增大或縮小。
     */
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // 因為tuple有實作Debug trait, 可用{:?}直接印出內容, {:#?}為pretty print
    println!("tup: {:?}", tup);
    println!("tup: {:#?}", tup);

    // 解構, destructure
    let (x,y, z) = tup;
    println!("x={}, y={}, z={}", x, y, z);

    // 不需要解構的位址可以_取代
    let (x,y, _) = tup;
    println!("x={}, y={}", x, y);

    // 以.與索引值直接存取tuple的元素
    println!("tup.0={}", tup.0);
    println!("tup.1={}", tup.1);
    println!("tup.2={}", tup.2);

    /*
    array, 陣列中的每個元素的類型必須相同
    array在stack配置空間
     */
    let a: [i32; 5]= [1, 2, 3, 4, 5];
    let b = [3; 5];
    println!("a={:?}", a);  // [1, 2, 3, 4, 5]
    println!("b={:?}", b);  // [3, 3, 3, 3, 3]
    println!("a[0]:{}", a[0]);  // 1

}
