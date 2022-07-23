use raylib::consts::KeyboardKey::*;
use raylib::prelude::*;
use std::process::Command;

struct Bounds {
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
}

fn main() {
    // Run in a closure scope so raylib closes when it leaves
    let run_event_loop = || -> (i32, i32) {
        let (mut rl, thread) = raylib::init()
            .transparent()
            .undecorated()
            .width(0)
            .height(0)
            .build();

        let (mut dist_x, mut dist_y) = (0, 0);

        let (x, y) = (rl.get_screen_width(), rl.get_screen_height());
        let mut b = Bounds {
            tl: (1, 1),
            tr: (x, 1),
            bl: (1, y),
            br: (x, y),
        };

        while !rl.window_should_close() {
            let center = b.get_center();
            dist_x = center.0 - rl.get_mouse_x();
            dist_y = center.1 - rl.get_mouse_y();

            match rl.get_key_pressed() {
                Some(KEY_H) => b.move_tl(),
                Some(KEY_J) => b.move_bl(),
                Some(KEY_K) => b.move_tr(),
                Some(KEY_L) => b.move_br(),
                Some(KEY_SPACE) => break,
                _ => {}
            }

            let mut d = rl.begin_drawing(&thread);
            d.clear_background(Color::new(0, 0, 0, 0));

            let center = b.get_center();
            d.draw_circle_lines(center.0, center.1, 5.0, Color::GREEN);

            draw_lines(d, &b);
        }

        (dist_x, dist_y)
    };

    let (dist_x, dist_y) = run_event_loop();

    if dist_x.abs() > 0 || dist_y.abs() > 0 {
        Command::new("wlrctl")
            .arg("pointer")
            .arg("move")
            .arg(dist_x.to_string())
            .arg(dist_y.to_string())
            .output()
            .expect("failed to execute pointer move");

        Command::new("wlrctl")
            .arg("pointer")
            .arg("click")
            .output()
            .expect("failed to execute pointer click");
    }
}

fn draw_lines(mut d: RaylibDrawHandle, b: &Bounds) {
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
