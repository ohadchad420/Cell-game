pub struct button {
    text: String,
    x: i32,
    y: i32,
    val: f32,
    minval: f32,
    maxval: f32
}


impl button {
    pub fn new(x: i32, y: i32, text: String, val: f32, minval: f32, maxval: f32) -> Self {
        button {x, y, text, val, minval, maxval}
    }

    pub fn gen_text(&self) -> String {
        let mut res: String = self.text.to_string();
        if self.minval != 0.0 && self.maxval != 0.0 {
            if self.val >= 0.0 && self.maxval <= 10.0 {  res = res + ":" + &format!("{:04.3}", self.val); }
            else if self.maxval > 10.0 { res = res + ":" + &format!("{:04.1}", self.val); }
            else {  res = res + ":" + &format!("{:05.3}", self.val); }
        }
        
        res
    }

    pub fn get_bar(&self) -> f32 {
        if self.minval != 0.0 && self.maxval != 0.0 {
            return (self.val - self.minval) / (self.maxval - self.minval);
        }
        return 0.0;
    }
 
    pub fn change_value(&mut self, new_val: f32) {
        if new_val > self.maxval { self.val = self.maxval; return; }
        if new_val < self.minval { self.val = self.minval; return; }
        self.val = new_val;
    }

    pub fn add_value(&mut self, add_val: f32) {
        if add_val + self.val > self.maxval { self.val = self.maxval; return; }
        if add_val + self.val < self.minval { self.val = self.minval; return; }
        self.val = add_val + self.val;
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    pub fn val(&self) -> f32 {
        self.val
    }

    pub fn minval(&self) -> f32 {
        self.minval
    }
    pub fn maxval(&self) -> f32 {
        self.maxval
    }
}