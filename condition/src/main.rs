fn main() {
    if_condition();
    if_as_setter();
    data_type_consistent();

}

fn if_condition() {
    let number = 3;

    // 必須使用邏輯判斷式，而不可用if number這種自動轉型的語法
    if number != 0 {
        println!("number was three");
    }
}

fn if_as_setter(){
    let n = 10;
    let x: i32 = if n >=5 { 1 } else { 10 };
    //--------------- ------^ -------- ^
    //------------------- 這兩個地方不要加分號, 做為expression
    println!("{}", x);
}

fn data_type_consistent(){
    let condition = true;

    let number = if condition {
        5
    } else {
        //"six"
        6
    };

    println!("The value of number is: {}", number);
}