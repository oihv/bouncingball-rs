use std::{
    ops::Add, thread::sleep, time
};

const WIDTH: usize = 32;
const HEIGHT: usize = 32;
// FPS = frame/second, 
const FPS: u32 = 60;

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
fn draw_circle(buff: &mut[[u8;WIDTH]; HEIGHT], cx: f32, cy: f32, r: f32) {
    for y in (cy - r).floor() as usize ..= (cy + r).ceil() as usize {
        for x in (cx - r).floor() as usize ..= (cx + r).ceil() as usize {
            if x > 0 && y > 0 && x < WIDTH && y < HEIGHT {
                let dx = (x as f32-cx) as i32;
                let dy = (y as f32-cy) as i32;
                let r = r as i32; 
                if dx.pow(2) + dy.pow(2) <= r.pow(2) {
                    buff[y][x] = b'@';
                }
            }
        }
    }
}

fn fill(buff: &mut[[u8;WIDTH]; HEIGHT], c: u8) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            buff[y][x] = c;
        }
    }
}

fn generate_string(string_vec: &mut Vec<String>, buff: &[[u8;WIDTH]; HEIGHT]) {
    string_vec.clear();
    for line in buff {
        string_vec.push(String::from_utf8((line[..WIDTH]).to_vec()).unwrap());
    }
}

fn back() {
    print!("\x1B[{WIDTH}D"); // move left
    print!("\x1B[{HEIGHT}A"); // move top
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
    let mut buff: [[u8; WIDTH]; HEIGHT] = [[0u8; HEIGHT]; WIDTH];
    let mut string_vec: Vec<String> = vec![String::from("anjay")];

    loop {
        fill(&mut buff, b' ');
        draw_circle(&mut buff, (WIDTH/2) as f32, (HEIGHT/2) as f32, 5.0);
        generate_string(&mut string_vec, &buff);
        show(&string_vec);
        sleep(time::Duration::from_secs(1)/FPS);
    }
}
