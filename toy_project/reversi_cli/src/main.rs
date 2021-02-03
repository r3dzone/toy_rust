use std::io;

static mut STONE_STATE: [[i8; 8]; 8] = [[3; 8]; 8]; //white:0 black:1 none:3

fn main() {
    unsafe {
        let mut turn_state = 0; //white:0 black:1
                                //stone initialize;
        STONE_STATE[3][3] = 0;
        STONE_STATE[3][4] = 1;
        STONE_STATE[4][3] = 1;
        STONE_STATE[4][4] = 0;

        title();

        loop {
            print_board();
            println!(
                "{} turn",
                if turn_state == 1 {
                    "black ○"
                } else {
                    "white ●"
                }
            );

            let mut turnover = true;
            let mut gameover = true;

            for i in 0..8 {
                for j in 0..8 {
                    if chk_chacksu(0, turn_state, &[i, j]) {
                        turnover = false;
                    }
                }
            }

            if turnover {
                turn_state = (turn_state + 1) % 2;
                for i in 0..8 {
                    for j in 0..8 {
                        if chk_chacksu(0, turn_state, &[i, j]) {
                            gameover = false;
                        }
                    }
                }
                if gameover {
                    break;
                }
                continue;
            }

            let ij: [usize; 2] = user_input();

            turn_state = if chk_chacksu(0, turn_state, &ij) {
                chk_chacksu(1, turn_state, &ij);
                STONE_STATE[ij[0]][ij[1]] = turn_state;
                (turn_state + 1) % 2
            } else {
                println!("can't put stones this location");
                turn_state
            };
        }
    }
    println!("gameover!");
}

unsafe fn print_board() {
    //black = ○ white = ●┼ ─ │
    let line = "┼───┼───┼───┼───┼───┼───┼───┼───┼";
    let col_idx = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'];
    for i in col_idx.iter() {
        print!("  {} ", i);
    }
    print!("\n");
    for i in 0..8 {
        print!(" {}\n{}", line, i + 1);
        for j in 0..8 {
            let stone = if STONE_STATE[i][j] == 0 {
                "●"
            } else if STONE_STATE[i][j] == 1 {
                "○"
            } else {
                " "
            };
            print!("│ {} ", stone);
        }
        print!("│\n");
    }
    println!(" {}", line);
}

fn user_input() -> [usize; 2] {
    println!("Please input the location you want to place: ");
    let mut loca = String::new();
    io::stdin()
        .read_line(&mut loca)
        .expect("fail to read input");
    let i = &loca[1..2];
    let j = &loca[0..1];

    let i: usize = match i.trim().parse() {
        Ok(i) => i,
        Err(_) => 9,
    };

    let j = match j {
        "a" => 0,
        "b" => 1,
        "c" => 2,
        "d" => 3,
        "e" => 4,
        "f" => 5,
        "g" => 6,
        "h" => 7,
        _ => 8,
    };

    if i == 9 || j == 8 {
        println!("please input 'low_case+number' ex) a2");
        return user_input();
    }

    let i = (i - 1) as usize;

    let ret: [usize; 2] = [i, j];
    return ret;
}

unsafe fn reverse_stone(mode: i8, prev: i8, xy: &[usize; 2], x: i8, y: i8) -> bool {
    //mode 0 = check 1 = reverse ,xy는 현재위치 xy[1] = x && xy[0]= y, x,y는 진행방향
    let now = STONE_STATE[xy[0]][xy[1]];
    if prev == now {
        let tmpx = xy[1] as i8 + x;
        let tmpy = xy[0] as i8 + y;
        if tmpx < 0 || tmpy < 0 || tmpx > 7 || tmpy > 7 {
            return false;
        }
        let tmpx = tmpx as usize;
        let tmpy = tmpy as usize;
        let flag = reverse_stone(mode, now, &[tmpy, tmpx], x, y);
        if mode == 1 && flag {
            STONE_STATE[xy[0]][xy[1]] = (now + 1) % 2;
        }
        return flag;
    } else {
        if now != 3 {
            return true;
        } else {
            return false;
        }
    }
}

unsafe fn chk_chacksu(mode: i8, turn_state: i8, ij: &[usize; 2]) -> bool {
    //mode 0 = check 1 = reverse
    let x: [i8; 8] = [-1, 0, 1, 1, 1, 0, -1, -1];
    let y: [i8; 8] = [1, 1, 1, 0, -1, -1, -1, 0];

    if STONE_STATE[ij[0]][ij[1]] != 3 {
        return false;
    }

    let mut flag = false;
    for i in 0..8 {
        let tmpx = ij[1] as i8 + x[i];
        let tmpy = ij[0] as i8 + y[i];
        if tmpx < 0 || tmpy < 0 || tmpx > 7 || tmpy > 7 {
            continue;
        }
        let tmpx = tmpx as usize;
        let tmpy = tmpy as usize;
        let enemy_stone = (turn_state + 1) % 2;

        if STONE_STATE[tmpy][tmpx] == enemy_stone
            && reverse_stone(mode, enemy_stone, &[tmpy, tmpx], x[i], y[i])
        {
            flag = true;
        }
    }

    return flag;
}

fn title() {
    let logo_aa = "\n______  _____  _   _  _____ ______  _____  _____ \n| ___ \\|  ___|| | | ||  ___|| ___ \\/  ___||_   _|\n| |_/ /| |__  | | | || |__  | |_/ /\\ `--.   | |\n|    / |  __| | | | ||  __| |    /  `--. \\  | |\n| |\\ \\ | |___ \\ \\_/ /| |___ | |\\ \\ /\\__/ / _| |_\n\\_| \\_|\\____/  \\___/ \\____/ \\_| \\_|\\____/  \\___/ \n\n";
    println!("{}", logo_aa);
}
