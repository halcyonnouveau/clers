use raylib::consts::KeyboardKey::*;
use raylib::prelude::*;
use std::process::Command;

const MOVE_SPEED: i32 = 2;

struct Bounds {
    screen_width: i32,
    screen_height: i32,
    // top-left (x, y)
    tl: (i32, i32),
    // top-right (x, y)
    tr: (i32, i32),
    // bottom-left (x, y)
    bl: (i32, i32),
    // bottom-right (x, y)
    br: (i32, i32),
}

impl Bounds {
    pub fn new(screen_width: i32, screen_height: i32) -> Bounds {
        Bounds {
            screen_width,
            screen_height,
            tl: (1, 1),
            tr: (screen_width, 1),
            bl: (1, screen_height),
            br: (screen_width, screen_height),
        }
    }

    pub fn mid(&self, num_1: i32, num_2: i32) -> i32 {
        (num_1 + num_2) / 2
    }

    pub fn get_center(&self) -> (i32, i32) {
        (
            self.mid(self.tl.0, self.tr.0),
            self.mid(self.tl.1, self.bl.1),
        )
    }

    pub fn move_tl(&mut self) {
        self.tr = (self.mid(self.tl.0, self.tr.0), self.tr.1);
        self.bl = (self.bl.0, self.mid(self.tl.1, self.bl.1));
        self.br = (
            self.mid(self.bl.0, self.br.0),
            self.mid(self.tr.1, self.br.1),
        );
    }

    pub fn move_tr(&mut self) {
        self.tl = (self.mid(self.tl.0, self.tr.0), self.tl.1);
        self.bl = (
            self.mid(self.bl.0, self.br.0),
            self.mid(self.tl.1, self.bl.1),
        );
        self.br = (self.br.0, self.mid(self.tr.1, self.br.1));
    }

    pub fn move_bl(&mut self) {
        self.tl = (self.tl.0, self.mid(self.tl.1, self.bl.1));
        self.tr = (
            self.mid(self.tl.0, self.tr.0),
            self.mid(self.tr.1, self.br.1),
        );
        self.br = (self.mid(self.bl.0, self.br.0), self.br.1);
    }

    pub fn move_br(&mut self) {
        self.tl = (
            self.mid(self.tl.0, self.tr.0),
            self.mid(self.tl.1, self.bl.1),
        );
        self.tr = (self.tr.0, self.mid(self.tr.1, self.br.1));
        self.bl = (self.mid(self.bl.0, self.br.0), self.bl.1);
    }

    pub fn move_up(&mut self) {
        if self.tl.1 > MOVE_SPEED {
            self.tl.1 -= MOVE_SPEED;
            self.tr.1 -= MOVE_SPEED;
            self.bl.1 -= MOVE_SPEED;
            self.br.1 -= MOVE_SPEED;
        }
    }

    pub fn move_down(&mut self) {
        if !(self.bl.1 > self.screen_height - MOVE_SPEED) {
            self.tl.1 += MOVE_SPEED;
            self.tr.1 += MOVE_SPEED;
            self.bl.1 += MOVE_SPEED;
            self.br.1 += MOVE_SPEED;
        }
    }

    pub fn move_left(&mut self) {
        if self.tl.0 > MOVE_SPEED {
            self.tl.0 -= MOVE_SPEED;
            self.tr.0 -= MOVE_SPEED;
            self.bl.0 -= MOVE_SPEED;
            self.br.0 -= MOVE_SPEED;
        }
    }

    pub fn move_right(&mut self) {
        if !(self.tr.0 > self.screen_width - MOVE_SPEED) {
            self.tl.0 += MOVE_SPEED;
            self.tr.0 += MOVE_SPEED;
            self.bl.0 += MOVE_SPEED;
            self.br.0 += MOVE_SPEED;
        }
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .transparent()
        .undecorated()
        .width(0)
        .height(0)
        .build();

    // Limit updates to slow down moving
    rl.set_target_fps(10);

    let mut click = false;
    let mut b = Bounds::new(rl.get_screen_width(), rl.get_screen_height());

    while !rl.window_should_close() {
        let center = b.get_center();
        let dist_x = center.0 - rl.get_mouse_x();
        let dist_y = center.1 - rl.get_mouse_y();

        if rl.is_key_down(KEY_LEFT_SHIFT) {
            // For key held down
            if rl.is_key_down(KEY_H) {
                b.move_left()
            }

            if rl.is_key_down(KEY_J) {
                b.move_down()
            }

            if rl.is_key_down(KEY_K) {
                b.move_up()
            }

            if rl.is_key_down(KEY_L) {
                b.move_right()
            }
        } else {
            // For key press and released
            match rl.get_key_pressed() {
                Some(KEY_H) => b.move_tl(),
                Some(KEY_J) => b.move_bl(),
                Some(KEY_K) => b.move_tr(),
                Some(KEY_L) => b.move_br(),
                Some(KEY_SEMICOLON) => {
                    move_mouse(dist_x, dist_y);
                    break;
                }
                Some(KEY_SPACE) => {
                    click = true;
                    move_mouse(dist_x, dist_y);
                    break;
                }
                _ => {}
            }
        }

        handle_draw(rl.begin_drawing(&thread), &b);
    }

    drop(rl);

    // Click after raylib window is closed, so it
    // actually clicks what you want
    if click {
        Command::new("wlrctl")
            .arg("pointer")
            .arg("click")
            .output()
            .expect("failed to execute pointer click");
    }
}

fn move_mouse(dist_x: i32, dist_y: i32) {
    Command::new("wlrctl")
        .arg("pointer")
        .arg("move")
        .arg(dist_x.to_string())
        .arg(dist_y.to_string())
        .output()
        .expect("failed to execute pointer move");
}

fn handle_draw(mut d: RaylibDrawHandle, b: &Bounds) {
    let center = b.get_center();
    d.clear_background(Color::new(0, 0, 0, 0));
    d.draw_circle_lines(center.0, center.1, 5.0, Color::GREEN);

    // (start_pos_x, start_pos_y, end_pos_x, end_pos_y, color);
    d.draw_line(b.tl.0, b.tl.1, b.tr.0, b.tr.1, Color::RED); // Top
    d.draw_line(b.tl.0, b.tl.1, b.bl.0, b.bl.1, Color::RED); // Left
    d.draw_line(b.bl.0, b.bl.1, b.br.0, b.br.1, Color::RED); // Bottom
    d.draw_line(b.tr.0, b.tr.1, b.br.0, b.br.1, Color::RED); // Right

    // Vertical
    d.draw_line(
        b.mid(b.tl.0, b.tr.0),
        b.tr.1,
        b.mid(b.bl.0, b.br.0),
        b.br.1,
        Color::RED,
    );

    // Horizontal
    d.draw_line(
        b.bl.0,
        b.mid(b.tl.1, b.bl.1),
        b.br.0,
        b.mid(b.tl.1, b.bl.1),
        Color::RED,
    );
}
