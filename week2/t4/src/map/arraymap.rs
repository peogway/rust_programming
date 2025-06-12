pub fn create_map() -> [[char; 5]; 5]{
    let mut map = [['*'; 5]; 5];
    for i in 0..5{
        for j in 0..5 {
            map[i][j] = '*';
        }
    }
    map[2][2] = 'x'; // Starting position
    map
}

pub fn move_up(pos: &mut i32) {
    if *pos < 5 {
        println!("Can't move out of the map");
    } else {
        *pos -= 5;
    }
}
pub fn move_down(pos: &mut i32) {
    if *pos >= 20 {
        println!("Can't move out of the map");
    } else {
        *pos+= 5
    }
}
pub fn move_left(pos: &mut i32) {
    if *pos % 5 == 0 {
        println!("Can't move out of the map");
    } else {
        *pos -= 1;
    }
}
pub fn move_right(pos: &mut i32) {
    if *pos % 5 == 4 {
        println!("Can't move out of the map");
    } else {
        *pos += 1;
    }
}
pub fn print_map(pos: i32) {
    let mut map = vec!['*'; 25];
    map[pos as usize] = 'x';
    for i in 0..5 {
        for j in 0..5 {
            print!("{}", map[i * 5 + j]);
            if j < 4 {
                print!(" ");
            }
        }
        println!();
    }
}

