fn main() {
    title();
    print_board();
}

fn print_board(){
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
            print!("│ {} ",j);
        }print!("│\n");
    }
    println!(" {}",line);
}

fn title(){
    let logo_aa = "\n______  _____  _   _  _____ ______  _____  _____ \n| ___ \\|  ___|| | | ||  ___|| ___ \\/  ___||_   _|\n| |_/ /| |__  | | | || |__  | |_/ /\\ `--.   | |\n|    / |  __| | | | ||  __| |    /  `--. \\  | |\n| |\\ \\ | |___ \\ \\_/ /| |___ | |\\ \\ /\\__/ / _| |_\n\\_| \\_|\\____/  \\___/ \\____/ \\_| \\_|\\____/  \\___/ \n\n";
    println!("{}",logo_aa);
}
