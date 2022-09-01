#[warn(dead_code)]

#[derive(Clone)]
pub struct atom {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
}

impl atom {
    pub fn new(x: f32, y: f32, vx: f32, vy: f32) -> Self {
        atom { x, y, vx, vy }
    }

    pub fn get_point(&self) -> sdl2::rect::Point {
        return sdl2::rect::Point::new(self.x as i32, self.y as i32);
    }
}

pub fn rule(group1: &mut [atom], group2: &[atom], G: f32, r: f32, s: f32) {
    for i in 0..group1.len() {
        let mut dx: f32 = 0.0;
        let mut dy: f32 = 0.0;
        for j in 0..group2.len() {
            let a: &atom = &group1[i];
            let b: &atom = &group2[j];

            let delx: f32 = a.x - b.x;
            let dely: f32 = a.y - b.y;
            let d: f32 = (delx * delx + dely * dely).sqrt();
            //println!("{}", d);
            if d > 0.0 && d < r {
                dx = dx + (G/d) * delx;
                dy = dy + (G/d) * dely;

                //println!("{} {}", dx, dy)
            }
        }

        group1[i].vx = (group1[i].vx + dx) * s;
        group1[i].vy = (group1[i].vy + dy) * s;
        //println!("{} {}", group1[i].vx, group1[i].vy);
        if (group1[i].x + group1[i].vx) < 25.0 || (group1[i].x + group1[i].vx) > 525.0 {
            group1[i].vx = -group1[i].vx;
        }
            group1[i].x = group1[i].x + group1[i].vx;

        if (group1[i].y + group1[i].vy) < 25.0 || (group1[i].y + group1[i].vy) > 525.0 {
            group1[i].vy = -group1[i].vy;
        }
        group1[i].y = group1[i].y + group1[i].vy;

        if group1[i].x > 525.0 || group1[i].x < 25.0 {
            group1[i].x = (((group1[i].x as i32 - 25) / 500) * 500 + 25) as f32;
        }

        if group1[i].y > 525.0 || group1[i].y < 25.0 {
            group1[i].y = (((group1[i].y as i32 - 25) / 500) * 500 + 25) as f32;
        }
        
    }
}