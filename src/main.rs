use rand::seq::SliceRandom;
use sdl2::pixels::Color;

mod atom;
mod font;
mod ui;

fn main() {
    let sdl2 = sdl2::init().unwrap();
    let mut rng = rand::thread_rng();

    let video = sdl2.video().unwrap();

    let HEIGHT: u32 = 550;
    let WIDTH: u32 = 720;

    let window = video.window("Life", WIDTH, HEIGHT).build()
    .expect("could not initialize video subsystem");

    let mut canvas = window.into_canvas().build().expect("could not make a canvas");
    let mut event_pump = sdl2.event_pump().unwrap();

    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
    canvas.present();

    //*************/
    //RULESET SETUP/
    //*************/

    let max_g: f32 = 4000.0;
    let max_s: f32 = 0.999;
    let min_r: f32 = 80.0;
    let max_r: f32 = 180.0;

    let atom_base_n: f32 = 250.0;
    let atom_max_n: f32 = 500.0;

    let base_fps: f32 = 30.0;

    let mut value_vec: Vec<f32> = vec![];

    //ORDER: color->red, color->green, color->blue, color->white, color:R
    for i in 0..20 {
        if (i + 1) % 5 == 0 {
            value_vec.push(rand::Rng::gen_range(&mut rng, min_r..max_r));                          //pushes R value every 5th index
        } else {
            value_vec.push((rand::Rng::gen_range(&mut rng, 0.0..max_g) - max_g / 2.0) / 10000.0);  //pushes G value
        }
    }

    value_vec.push(rand::Rng::gen_range(&mut rng, 1.0..max_s * 1000.0) / 1000.0);                  //pushes speed
    value_vec.push(base_fps);                                                                            //pushes fps

    for _ in 0..4 {
        value_vec.push(atom_base_n);                                                                //pushes # for each atom
    }

    let mut red_atoms: Vec<atom::atom> = init_atoms(value_vec[22] as usize, &mut rng);
    let mut green_atoms: Vec<atom::atom> = init_atoms(value_vec[23] as usize, &mut rng);
    let mut blue_atoms: Vec<atom::atom> = init_atoms(value_vec[24] as usize, &mut rng);
    let mut white_atoms: Vec<atom::atom> = init_atoms(value_vec[25] as usize, &mut rng);

    //***************/
    //INTERFACE SETUP/
    //***************/

    let max_g_button: f32 = max_g / 20000.0;

    canvas.set_draw_color(Color::WHITE);
    canvas.fill_rect(sdl2::rect::Rect::new(22,22,506,506)).unwrap();

    //order for each color => red, green, blue, white, radius
    let red_b1: ui::button = ui::button::new(540, 9 + 18 * 0, "red > red".to_string(), value_vec[0], -max_g_button, max_g_button);
    render_button(&red_b1, &mut canvas, false);
    let red_b2: ui::button = ui::button::new(540, 9 + 18 * 1, "red > green".to_string(), value_vec[1], -max_g_button, max_g_button);
    render_button(&red_b2, &mut canvas, false);
    let red_b3: ui::button = ui::button::new(540, 9 + 18 * 2, "red > blue".to_string(), value_vec[2], -max_g_button, max_g_button);
    render_button(&red_b3, &mut canvas, false);
    let red_b4: ui::button = ui::button::new(540, 9 + 18 * 3, "red > white".to_string(), value_vec[3], -max_g_button, max_g_button);
    render_button(&red_b4, &mut canvas, false);
    let red_b5: ui::button = ui::button::new(540, 9 + 18 * 4, "red-r".to_string(), value_vec[4], min_r, max_r);
    render_button(&red_b5, &mut canvas, false);

    let green_b1: ui::button = ui::button::new(540, 103 + 18 * 0, "green > red".to_string(), value_vec[5], -max_g_button, max_g_button);
    render_button(&green_b1, &mut canvas, false);
    let green_b2: ui::button = ui::button::new(540, 103 + 18 * 1, "green > green".to_string(), value_vec[6], -max_g_button, max_g_button);
    render_button(&green_b2, &mut canvas, false);
    let green_b3: ui::button = ui::button::new(540, 103 + 18 * 2, "green > blue".to_string(), value_vec[7], -max_g_button, max_g_button);
    render_button(&green_b3, &mut canvas, false);
    let green_b4: ui::button = ui::button::new(540, 103 + 18 * 3, "green > white".to_string(), value_vec[8], -max_g_button, max_g_button);
    render_button(&green_b4, &mut canvas, false);
    let green_b5: ui::button = ui::button::new(540, 103 + 18 * 4, "green-r".to_string(), value_vec[9], min_r, max_r);
    render_button(&green_b5, &mut canvas, false);
 
    let blue_b1: ui::button = ui::button::new(540, 197 + 18 * 0, "blue > red".to_string(), value_vec[10], -max_g_button, max_g_button);
    render_button(&blue_b1, &mut canvas, false);
    let blue_b2: ui::button = ui::button::new(540, 197 + 18 * 1, "blue > green".to_string(), value_vec[11], -max_g_button, max_g_button);
    render_button(&blue_b2, &mut canvas, false);
    let blue_b3: ui::button = ui::button::new(540, 197 + 18 * 2, "blue > blue".to_string(), value_vec[12], -max_g_button, max_g_button);
    render_button(&blue_b3, &mut canvas, false);
    let blue_b4: ui::button = ui::button::new(540, 197 + 18 * 3, "blue > white".to_string(), value_vec[13], -max_g_button, max_g_button);
    render_button(&blue_b4, &mut canvas, false);
    let blue_b5: ui::button = ui::button::new(540, 197 + 18 * 4, "blue-r".to_string(), value_vec[14], min_r, max_r);
    render_button(&blue_b5, &mut canvas, false);
 
    let white_b1: ui::button = ui::button::new(540, 291 + 18 * 0, "white > red".to_string(), value_vec[15], -max_g_button, max_g_button);
    render_button(&white_b1, &mut canvas, false);
    let white_b2: ui::button = ui::button::new(540, 291 + 18 * 1, "white > green".to_string(), value_vec[16], -max_g_button, max_g_button);
    render_button(&white_b2, &mut canvas, false);
    let white_b3: ui::button = ui::button::new(540, 291 + 18 * 2, "white > blue".to_string(), value_vec[17], -max_g_button, max_g_button);
    render_button(&white_b3, &mut canvas, false);
    let white_b4: ui::button = ui::button::new(540, 291 + 18 * 3, "white > white".to_string(), value_vec[18], -max_g_button, max_g_button);
    render_button(&white_b4, &mut canvas, false); 
    let white_b5: ui::button = ui::button::new(540, 291 + 18 * 4, "white-r".to_string(), value_vec[19], min_r, max_r);
    render_button(&white_b5, &mut canvas, false);

    //speed of particles
    let general_b1: ui::button = ui::button::new(540, 385 + 18 * 0, "speed".to_string(), value_vec[20], 0.001, max_s);
    render_button(&general_b1, &mut canvas, false); 
    //time between frames
    let general_b2: ui::button = ui::button::new(540, 385 + 18 * 1, "update".to_string(), value_vec[21] as f32, 5.0, 60.0);
    render_button(&general_b2, &mut canvas, false);

    let count_b1: ui::button = ui::button::new(540, 425 + 18 * 0, "red #".to_string(), value_vec[22] as f32, -0.001, 500.0);
    render_button(&count_b1, &mut canvas, false);
    let count_b2: ui::button = ui::button::new(540, 425 + 18 * 1, "green #".to_string(), value_vec[23] as f32, -0.001, 500.0);
    render_button(&count_b2, &mut canvas, false);
    let count_b3: ui::button = ui::button::new(540, 425 + 18 * 2, "blue #".to_string(), value_vec[24] as f32, -0.001, 500.0);
    render_button(&count_b3, &mut canvas, false);
    let count_b4: ui::button = ui::button::new(540, 425 + 18 * 3, "white #".to_string(), value_vec[25] as f32, -0.001, 500.0);
    render_button(&count_b4, &mut canvas, false);

    canvas.set_draw_color(Color::RED);
    canvas.fill_rect(sdl2::rect::Rect::new(533,10,4,18*5 - 1)).unwrap();
    canvas.set_draw_color(Color::GREEN);
    canvas.fill_rect(sdl2::rect::Rect::new(533,104,4,18*5 - 1)).unwrap();
    canvas.set_draw_color(Color::BLUE);
    canvas.fill_rect(sdl2::rect::Rect::new(533,197,4,18*5 - 1)).unwrap();
    canvas.set_draw_color(Color::WHITE);
    canvas.fill_rect(sdl2::rect::Rect::new(533,291,4,18*5 - 1)).unwrap();
    canvas.set_draw_color(Color::RGB(75, 255, 180));
    canvas.fill_rect(sdl2::rect::Rect::new(533,386,4,425 + 18*4 - 386 - 1)).unwrap();

    draw_string(&"made by:", 545, 505, &mut canvas);
    draw_string(&"> ohadchad420", 545, 505 + 12, &mut canvas);

    draw_string(&"^/~:move between tabs. </>:change value. LSHIFT:higher precision.", 8, 7, &mut canvas);
    draw_string(&"R:randomize values. +LSHIFT:reinit atoms. +CTRL:keep values and reinit atoms.", 8, 535, &mut canvas);
    

    let mut button_vec: Vec<ui::button> = vec![ red_b1, red_b2, red_b3, red_b4, red_b5,
                                            green_b1, green_b2, green_b3, green_b4, green_b5,
                                            blue_b1, blue_b2, blue_b3, blue_b4, blue_b5,
                                            white_b1, white_b2, white_b3, white_b4, white_b5,
                                            general_b1, general_b2,
                                            count_b1, count_b2, count_b3, count_b4];

    let mut button_index: usize = 0;

    render_button(&button_vec[button_index], &mut canvas, true);

    canvas.present();

    let mut ctrl_pressed: bool = false;
    let mut shift_pressed: bool = false;

    let mut count_cache = vec![value_vec[22], value_vec[23], value_vec[24], value_vec[25]];

    'run: loop {

        let start = std::time::Instant::now();

        canvas.present();

        atom::rule(&mut red_atoms, &green_atoms, value_vec[1], value_vec[4], value_vec[20]);
        atom::rule(&mut red_atoms, &blue_atoms, value_vec[2], value_vec[4], value_vec[20]);
        atom::rule(&mut red_atoms, &white_atoms, value_vec[3], value_vec[4], value_vec[20]);

        let red_cache = red_atoms.clone();
        atom::rule(&mut red_atoms, &red_cache, value_vec[0], value_vec[4], value_vec[20]);
        

        atom::rule(&mut green_atoms, &red_atoms, value_vec[5], value_vec[9], value_vec[20]);
        atom::rule(&mut green_atoms, &blue_atoms, value_vec[7], value_vec[9], value_vec[20]);
        atom::rule(&mut green_atoms, &white_atoms, value_vec[8], value_vec[9], value_vec[20]);

        let green_cache = green_atoms.clone();
        atom::rule(&mut green_atoms, &green_cache, value_vec[6], value_vec[9], value_vec[20]);
        

        atom::rule(&mut blue_atoms, &red_atoms, value_vec[10], value_vec[14], value_vec[20]);
        atom::rule(&mut blue_atoms, &green_atoms, value_vec[11], value_vec[14], value_vec[20]);
        atom::rule(&mut blue_atoms, &white_atoms, value_vec[13], value_vec[14], value_vec[20]);

        let blue_cache = blue_atoms.clone();
        atom::rule(&mut blue_atoms, &blue_cache, value_vec[12], value_vec[14], value_vec[20]);

        atom::rule(&mut white_atoms, &red_atoms, value_vec[15], value_vec[19], value_vec[20]);
        atom::rule(&mut white_atoms, &green_atoms, value_vec[16], value_vec[19], value_vec[20]);
        atom::rule(&mut white_atoms, &blue_atoms, value_vec[17], value_vec[19], value_vec[20]);

        let white_cache = white_atoms.clone();
        atom::rule(&mut white_atoms, &white_cache, value_vec[18], value_vec[19], value_vec[20]);

        canvas.set_draw_color(Color::BLACK);
        canvas.fill_rect(sdl2::rect::Rect::new(24,24,502,502)).unwrap();

        canvas.set_draw_color(Color::RED);
        for i in 0..count_cache[0] as usize {
            canvas.draw_point(red_atoms[i].get_point()).unwrap();
        }

        canvas.set_draw_color(Color::GREEN);
        for i in 0..count_cache[1] as usize {
            canvas.draw_point(green_atoms[i].get_point()).unwrap();
        }

        canvas.set_draw_color(Color::BLUE);
        for i in 0..count_cache[2] as usize {
            canvas.draw_point(blue_atoms[i].get_point()).unwrap();
        }

        canvas.set_draw_color(Color::WHITE);
        for i in 0..count_cache[3] as usize {
            canvas.draw_point(white_atoms[i].get_point()).unwrap();
        }

        

        canvas.present();

        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} |
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Escape), .. } => {
                    break 'run;
                    },
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Down), .. } => {
                        render_button(&button_vec[button_index], &mut canvas, false);
                        button_index = button_index + 1;
                        if button_index == button_vec.len() { button_index = 0; }
                        render_button(&button_vec[button_index], &mut canvas, true);
                    },
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Up), .. } => {
                        render_button(&button_vec[button_index], &mut canvas, false);
                        if button_index == 0 { button_index = button_vec.len(); }
                        button_index = button_index - 1;
                        render_button(&button_vec[button_index], &mut canvas, true);
                    },
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::R), .. } => {
                        if !ctrl_pressed {
                            //println!("{}", speed);
                            for i in 0..20 {
                                if (i + 1) % 5 == 0 {
                                    value_vec[i] = rand::Rng::gen_range(&mut rng, min_r..max_r);                          //pushes R value every 5th index
                                } else {
                                    value_vec[i] = (rand::Rng::gen_range(&mut rng, 0.0..max_g) - max_g / 2.0) / 10000.0;  //pushes G value
                                }
                            }
                            value_vec[20] = rand::Rng::gen_range(&mut rng, 1.0..max_s * 1000.0) / 1000.0;                  //pushes speed

                            for i in 0..value_vec.len() {
                                button_vec[i].change_value(value_vec[i]);
                                render_button(&button_vec[i], &mut canvas, false);
                            }
                            render_button(&button_vec[button_index], &mut canvas, true);
                        }
                        if shift_pressed  || ctrl_pressed {
                            
                            count_cache = vec![value_vec[22], value_vec[23], value_vec[24], value_vec[25]];

                            red_atoms = vec![];
                        
                            if value_vec[22] > 0.0 {
                                for _ in 0..value_vec[22] as usize {
                                    red_atoms.push(atom::atom::new(rand::Rng::gen_range(&mut rng, 25.0..525.0), 
                                    rand::Rng::gen_range(&mut rng, 25.0..525.0), 
                                    0.0, 0.0));
                                }
                            }
                        
                            green_atoms = vec![];
                        
                            if value_vec[23] > 0.0 {
                                for _ in 0..value_vec[23] as usize {
                                    green_atoms.push(atom::atom::new(rand::Rng::gen_range(&mut rng, 25.0..525.0), 
                                    rand::Rng::gen_range(&mut rng, 25.0..525.0), 
                                    0.0, 0.0));
                                }
                            }
                        
                            blue_atoms = vec![];
                        
                            if value_vec[24] > 0.0 {
                                for _ in 0..value_vec[24] as usize {
                                    blue_atoms.push(atom::atom::new(rand::Rng::gen_range(&mut rng, 25.0..525.0), 
                                    rand::Rng::gen_range(&mut rng, 25.0..525.0), 
                                    0.0, 0.0));
                                }
                            }
                        
                            white_atoms = vec![];
                        
                            if value_vec[25] > 0.0 {
                                for _ in 0..value_vec[25] as usize {
                                    white_atoms.push(atom::atom::new(rand::Rng::gen_range(&mut rng, 25.0..525.0), 
                                    rand::Rng::gen_range(&mut rng, 25.0..525.0), 
                                    0.0, 0.0));
                                }
                            }
                        }
                    }
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Left), .. } => {
                        let mut precision: u8 = 1;
                        if(shift_pressed) { precision = 10; }
                        if (button_index + 1) % 5 == 0 && button_index < 20 { //radius button, change by 1 or 0.1
                            value_vec[button_index] = change_active_button(-1.0, precision, &mut button_vec[button_index]);
                        } else if (button_index + 1) % 5 != 0  && button_index <= 20{ //force / speed button change by 0.01 or 0.001
                            value_vec[button_index] = change_active_button(-0.01, precision, &mut button_vec[button_index]);
                        } else if button_index >= 21 { //fps / count button, change by 1
                            //println!("{}", value_vec[button_index]);
                            value_vec[button_index] = change_active_button(-10.0, precision, &mut button_vec[button_index]);
                        }
                        if value_vec[button_index] < button_vec[button_index].minval() {
                            value_vec[button_index] = button_vec[button_index].minval();
                        }
                        render_button(&button_vec[button_index], &mut canvas, true);
                    }, 
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Right), .. } => {
                        let mut precision: u8 = 1;
                        if(shift_pressed) { precision = 10; }
                        if (button_index + 1) % 5 == 0 && button_index < 20 { //radius button, change by 1 or 0.1
                            value_vec[button_index] = change_active_button(1.0, precision, &mut button_vec[button_index]);
                        } else if (button_index + 1) % 5 != 0  && button_index <= 20{ //force / speed button change by 0.01 or 0.001
                            value_vec[button_index] = change_active_button(0.01, precision, &mut button_vec[button_index]);
                        } else if button_index >= 21 { //fps / count button, change by 1
                            //println!("{}", value_vec[button_index]);
                            value_vec[button_index] = change_active_button(10.0, precision, &mut button_vec[button_index]);
                        }
                        if value_vec[button_index] > button_vec[button_index].maxval() {
                            value_vec[button_index] = button_vec[button_index].maxval();
                        }
                        render_button(&button_vec[button_index], &mut canvas, true);
                    }, 
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::LShift), .. } => {
                        shift_pressed = true;
                    },
    
                    sdl2::event::Event::KeyUp { keycode: Some(sdl2::keyboard::Keycode::LShift), .. } => {
                        shift_pressed = false;
                    },
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::LCtrl), .. } => {
                        ctrl_pressed = true;
                    },
    
                    sdl2::event::Event::KeyUp { keycode: Some(sdl2::keyboard::Keycode::LCtrl), .. } => {
                        ctrl_pressed = false;
                    },
                    _ => {}
            }
        }

        let wait_time = start.elapsed();
        
        if wait_time < std::time::Duration::new(0, 1000000000 / value_vec[21] as u32) {
            
            std::thread::sleep(std::time::Duration::new(0, 1000000000 / value_vec[21] as u32) - wait_time);
        }
    }
}

fn init_atoms(n: usize, rng: &mut rand::rngs::ThreadRng) -> Vec<atom::atom> {
    let mut res: Vec<atom::atom> = vec![];
    for _ in 0..n {
        res.push(atom::atom::new(rand::Rng::gen_range(rng, 25.0..525.0), 
        rand::Rng::gen_range(rng, 25.0..525.0), 
        0.0, 0.0));
    }
    return res;
}

fn draw_string(s: &str, x: i32, y: i32, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
    for h in 0..s.chars().count() {
        let ls = font::get_letter(s.chars().nth(h).unwrap());
        canvas.set_draw_color(Color::WHITE);
        for i in 0..8 {
            for j in 0..8 {
                if ls[i][j] {
                    canvas.draw_point(sdl2::rect::Point::new(x + j as i32 + (h as i32 * 8), y + i as i32)).unwrap();
                }
            }
        }
    }
}

fn draw_char(c: char, x: i32, y: i32, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
    let ls = font::get_letter(c);
    canvas.set_draw_color(Color::WHITE);
    for i in 0..8 {
        for j in 0..8 {
            if ls[i][j] {
                canvas.draw_point(sdl2::rect::Point::new(x + j as i32, y + i as i32)).unwrap();
            }
        }
    }
}

pub fn render_button(button: &ui::button, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, is_pressed: bool) {
    let width: u32 = 170;
    let height: u32 = 18; //1st and last pixel are empty

    if is_pressed { 
        canvas.set_draw_color(Color::RGB(180, 180, 255)); }
    else { canvas.set_draw_color(Color::WHITE); }
    canvas.fill_rect(sdl2::rect::Rect::new(button.x() + 1, button.y() + 1, width - 2, height - 2)).unwrap();

    canvas.set_draw_color(Color::BLACK);
    canvas.fill_rect(sdl2::rect::Rect::new(button.x() + 3, button.y() + 3, width - 6, height - 6)).unwrap();

    canvas.set_draw_color(Color::GRAY);
    //println!("{}", button.get_bar());
    if button.get_bar() > 0.0 {
        canvas.fill_rect(sdl2::rect::Rect::new(button.x() + 3, button.y() + 3, ((width as f32 - 6.0) * button.get_bar()) as u32, height - 6)).unwrap();
    }

    //println!("{}", button.gen_text());
    canvas.set_draw_color(Color::WHITE);
    for i in 0..button.gen_text().chars().count() {
        draw_char(button.gen_text().chars().nth(i).unwrap(), button.x() + 5 + i as i32 * 8, button.y() + 5, canvas);
    }
}

fn change_active_button(val: f32, precision: u8, button: &mut ui::button) -> f32 {
    let change_val = val / precision as f32;
    button.add_value(change_val);
    return button.val();
}