const MAX: u32 = 100_000; //const

fn main() {
    let mut x = 5; //mutable
    let y = 3; //not mutable
    println!("x value is {}", x);
    println!("y value is {}", y);
    x = 6;
    println!("x value is {}", x);
    let y = y + 1;
    println!("y value is {}", y);
    print!("MAX value is {}\n", MAX);
    let space = "   ";
    let space = space.len(); //형변환(?) 쩐다!(shadowing)
                             //mut일 경우는 형이 다르다고 거부됨
    println!("space length is {}", space);
    let z = 0b10;
    println!("z value is {}", z);
    let z = 2.0;
    println!("z value is {}", z);
    let flag: bool = true; //boolean
    println!("flag value is {}", flag);

    //compound type
    let tup: (i32, u64, f64) = (242, 528, 4.5); //tuple
    println!("second value is {}", tup.1); //직접호출
    let (a, b, c) = tup; //구조해체
    println!("b value is {}", b);
    let arr = [1, 2, 3, 4, 5]; //array
    let idx = 3;
    let element = arr[idx];
    println!("fourth value is {}", element);
    println!("return is {}",my_func(idx));

    //표현식
    let y = {
        let x = 3;
        x + 1 //표현식 끝에는 ;안붙음
              //;붙이면 구문으로 변경되면서 반환값 X
    };
}

fn my_func(tmp: usize) -> i32{
    print!("hello func! tmp is {}\n", tmp);
    5
}
