fn main() {
    let num = 3;

    if num < 5 {
        //if의 조건은 무조건 boolean 타입이어야 함
        println!("condition is true!");
    } else {
        println!("condition is false!");
    }

    let condition = true;
    let num = if condition { 5 } else { 6 };

    println!("number is {}", num);

    // loop{
    //  println!("muhan loop");
    // }

    let mut num = num;

    while num != 0 {
        println!("looped {}times", 5 - num + 1);
        num = num - 1;
    }

    let arr = [1, 3, 5, 7, 11];

    for element in arr.iter() {
        println!("array value is: {}", element);
    }

    for num in (1..6).rev() {
        println!("{}", num);
    }
}
