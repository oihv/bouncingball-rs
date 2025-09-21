use std::{
   ops::Add, process::exit, thread::sleep, time
};

const WIDTH: usize = 64;
const HEIGHT: usize = 64;
// FPS = frame/second, 
const FPS: u32 = 60;

#[derive(Copy, Clone)]
enum Color {
    Empty,
    Fill
}

#[derive(Debug, Clone, PartialEq)]
struct Vec2<T> {
    x: T,
    y: T
}

impl Vec2<f32> {
    fn add(&self, rhs: &Vec2<f32>) -> Vec2<f32> {
        let mut res = Vec2::<f32>{x: 0.0, y: 0.0};
        res.x = self.x + rhs.x;
        res.y = self.y + rhs.y;
        res
    }

    fn subs(&self, rhs: &Vec2<f32>) -> Vec2<f32> {
        let mut res = Vec2::<f32>{x: 0.0, y: 0.0};
        res.x = self.x - rhs.x;
        res.y = self.y - rhs.y;
        res
    }

    fn scale(&self, rhs: f32) -> Vec2<f32> {
        let mut res = Vec2::<f32>{x: 0.0, y: 0.0};
        res.x = self.x * rhs;
        res.y = self.y * rhs;
        res
    }
}

fn get_color_code(c: Color) -> u8 {
    match c {
        Color::Empty => 0,
        Color::Fill => 1,
    }
}

fn get_fill(num: u8) -> u8 {
    match num {
        0 => b' ', // Empty
        1 => b'_', // Underline
        2 => b'^', // Top line 
        3 => b'@', // Fill
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
fn draw_circle(buff: &mut[[Color;WIDTH]; HEIGHT], c: &Vec2<f32>, r: f32) {
    for y in (c.y - r).floor() as usize ..= (c.y + r).ceil() as usize {
        for x in (c.x - r).floor() as usize ..= (c.x + r).ceil() as usize {
            if x > 0 && y > 0 && x < WIDTH && y < HEIGHT {
                let dx = (x as f32-c.x) as i32;
                let dy = (y as f32-c.y) as i32;
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

fn back_cursor() {
    print!("\x1B[{WIDTH}D"); // move left
    print!("\x1B[{}A", HEIGHT / 2); // move top
}

fn show(string_vec: &Vec<String>) {
    for line in string_vec {
        println!("{line}");
    }
}

const GRAVITY: f32 = 9.8;
const DT: f32 = 0.05;
const ELASTICITY: f32 = -0.8;
const FRICTION: f32 = 0.07;

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

    let mut pos = Vec2::<f32>{x: (WIDTH as f32)/2.0, y: 0.0};
    let mut prev_pos = Vec2::<f32>{x: (WIDTH as f32)/2.0, y: 0.0};

    let mut v = Vec2::<f32>{x: 5.0, y: 0.0};
    let r: f32 = 10.0;

    let mut stable_count = 0;

    loop {
        fill(&mut buff);
        draw_circle(&mut buff, &pos, r);

        // Gravity effects velocity
        v.y += GRAVITY * DT;

        // Add velocity to position
        pos = pos.add(&v.scale(DT));

        // Ground collision
        if pos.y + r > HEIGHT as f32 {
            v.y *= ELASTICITY;
            pos.y = HEIGHT as f32 - r;

            v.x -= FRICTION * v.x;
        }
        
        // Wall collision
        if pos.x - r < 0.0 {
            pos.x = r;
            v.x *= ELASTICITY;
        } else if pos.x + r > WIDTH as f32 {
            pos.x = WIDTH as f32 - r;
            v.x *= ELASTICITY;
        }

        // Count when to stop
        if pos == prev_pos {
            stable_count += 1;
        } else {
            stable_count = 0;
        }

        generate_string(&mut string_vec, &buff);
        show(&string_vec);

        if stable_count == 30 {
            break;
        }

        back_cursor();
        sleep(time::Duration::from_secs(1)/FPS);
        prev_pos = pos.clone();
    }
}
