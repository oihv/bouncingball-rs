use std::{
    thread::sleep, time
};

const WIDTH: usize = 32;
const HEIGHT: usize = 32;
// FPS = frame/second, 
const FPS: u32 = 60;

#[derive(Copy, Clone)]
enum Color {
    Empty,
    Fill
}

fn get_color_code(c: Color) -> u8 {
    match c {
        Color::Empty => 0,
        Color::Fill => 1,
    }
}

fn get_fill(num: u8) -> u8 {
    match num {
        0 => b' ',
        1 => b'_',
        2 => b'-',
        3 => b'=',
        4_u8..=u8::MAX => b'X',
    }
}
/* r = 3
* ---------------
* ----#######----
* ----#-@@@-#----
* ----#@@@@@#----
* ----#@@c@@#----
* ----#@@@@@#----
* ----#-@@@-#----
* ----#######----
* ---------------
* x^2 + y^2 <= r^2
*/
fn draw_circle(buff: &mut[[Color;WIDTH]; HEIGHT], cx: f32, cy: f32, r: f32) {
    for y in (cy - r).floor() as usize ..= (cy + r).ceil() as usize {
        for x in (cx - r).floor() as usize ..= (cx + r).ceil() as usize {
            if x > 0 && y > 0 && x < WIDTH && y < HEIGHT {
                let dx = (x as f32-cx) as i32;
                let dy = (y as f32-cy) as i32;
                let r = r as i32; 
                if dx.pow(2) + dy.pow(2) <= r.pow(2) {
                    buff[y][x] = Color::Fill;
                }
            }
        }
    }
}

fn fill(buff: &mut[[Color;WIDTH]; HEIGHT]) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            buff[y][x] = Color::Empty;
        }
    }
}

// Implements double row compression
fn generate_string(string_vec: &mut Vec<String>, buff: &[[Color;WIDTH]; HEIGHT]) {
    string_vec.clear();
    let mut lines = buff.iter();
 
    while let Some(line) = lines.next() {
        let mut row: [u8; WIDTH] = [0u8; WIDTH];

        let next_line = lines.next().expect("Width is probably not even");

        for i in 0..WIDTH {
            let top = get_color_code(line[i]);
            let bottom = get_color_code(next_line[i]);

            row[i] = get_fill(top*2 + bottom);
        }

        string_vec.push(String::from_utf8((row[..WIDTH]).to_vec()).unwrap());
    }
}

fn back() {
    print!("\x1B[{WIDTH}D"); // move left
    print!("\x1B[{}A", HEIGHT / 2); // move top
}

fn show(string_vec: &Vec<String>) {
    for line in string_vec {
        println!("{line}");
    }
    back();
}

fn main() {
    // [ ------WIDTH------
    // [x, x, x, x], |
    // [x, x, x, x], |
    // [x, x, x, x], HEIGHT
    // [x, x, x, x], |
    // [x, x, x, x], |
    // ]
    let mut buff: [[Color; WIDTH]; HEIGHT] = [[Color::Empty; HEIGHT]; WIDTH];
    let mut string_vec: Vec<String> = vec![String::from("anjay")];

    loop {
        fill(&mut buff);
        draw_circle(&mut buff, (WIDTH/2) as f32, (HEIGHT/2) as f32, 5.0);
        generate_string(&mut string_vec, &buff);
        show(&string_vec);
        sleep(time::Duration::from_secs(1)/FPS);
    }
}
