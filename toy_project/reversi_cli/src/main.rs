use std::io;

fn main() {
   let mut stone_state :[[i8;8];8] = [[3;8];8]; //white:0 black:1 none:3

    //stone initialize;
    stone_state[3][3] = 0;
    stone_state[3][4] = 1;
    stone_state[4][3] = 1;
    stone_state[4][4] = 0;

    title();

    loop{
         let mut loca = String::new();
        println!("Please input the location you want to place: ");
        io::stdin().read_line(&mut loca).expect("fail to read input");
        let j = &loca[0..1];
        let j = match j{
            "a" => 0 ,
            "b" => 1 ,
            "c" => 2 ,
            "d" => 3 ,
            "e" => 4 ,
            "f" => 5 ,
            "g" => 6 ,
            "h" => 7 ,
            _ => continue,
        };
        let i = &loca[1..2];
        let i: i8 = match i.trim().parse() {
            Ok(i) => i,
            Err(_) => continue,
        };
        let i = i -1;
        println!("{}",i);
        stone_state[i as usize][j] = 0;
        print_board(&stone_state);
    }
}

fn print_board(stone_state : &[[i8;8];8]){
    //black = ● white = ○ ┼ ─ │
    let line = "┼───┼───┼───┼───┼───┼───┼───┼───┼";
    let col_idx = ['A','B','C','D','E','F','G','H'];
    for i in col_idx.iter(){
        print!("  {} ",i);
    }
    print!("\n");
    for i in 0..8 {
        print!(" {}\n{}",line,i+1);
        for j in 0..8{
            let stone = if stone_state[i][j] == 0 {
                "○"
            }else if stone_state[i][j] == 1{
                "●"
            }else{
                " "
            };
            print!("│ {} ",stone);
        }print!("│\n");
    }
    println!(" {}",line);
}

fn title(){
    let logo_aa = "\n______  _____  _   _  _____ ______  _____  _____ \n| ___ \\|  ___|| | | ||  ___|| ___ \\/  ___||_   _|\n| |_/ /| |__  | | | || |__  | |_/ /\\ `--.   | |\n|    / |  __| | | | ||  __| |    /  `--. \\  | |\n| |\\ \\ | |___ \\ \\_/ /| |___ | |\\ \\ /\\__/ / _| |_\n\\_| \\_|\\____/  \\___/ \\____/ \\_| \\_|\\____/  \\___/ \n\n";
    println!("{}",logo_aa);
}
