use gdnative::prelude::*;
use std::time::{SystemTime};

#[derive(NativeClass)]
#[inherit(Node2D)]
struct HelloWorld{
    colorsarray: [Color; 9]
}


#[methods]
impl HelloWorld {
    fn new(_owner: &Node2D) -> Self {
        HelloWorld{
            colorsarray:[
                Color::from_rgba(0.0, 0.0, 0.0, 1.0),
                Color::from_rgba(1.0, 0.0, 1.0, 1.0),
                Color::from_rgba(0.0, 0.0, 1.0, 1.0),
                Color::from_rgba(0.0, 1.0, 0.0, 1.0),
                Color::from_rgba(0.0, 0.5, 1.0, 1.0),
                Color::from_rgba(1.0, 0.0, 0.0, 1.0),
                Color::from_rgba(1.0, 0.5, 0.0, 1.0),
                Color::from_rgba(1.0, 1.0, 1.0, 1.0),
                Color::from_rgba(1.0, 1.0, 1.0, 1.0)]

        }
    }


    #[export]
    fn _ready(&self, _owner: &Node2D) {
        //godot_print!("hi, this is _ready.");
    }


    #[export]
    fn _draw(&self, owner: &Node2D) {
        let sys_time = SystemTime::now();
        let mut c : Color;
        let l : f32=100.0;
        for i in 0..499 {
            for j in 0..499 {
                let mut u:f32=(i as f32) / 250.0 - 1.5;
                let mut v:f32=(j as f32) / 250.0 - 1.0;
                let mut x:f32=u;
                let mut y:f32=v;
                let mut n:f32=0.0;
                let mut r:f32=x*x;
                let mut q:f32=y*y;
                while (r+q)<4.0 && n<l {
                    y=2.0*x*y+v;
                    x=r-q+u;
                    r=x*x;
                    q=y*y;
                    n=n+1.0;
                }
                if n<10.0 {
                    c=self.colorsarray[0];
                }
                else{
                    c=self.colorsarray[(8.0 * (n - 10.0) / (l - 10.0)).round() as usize];
                }
                owner.draw_rect(Rect2{position: Vector2{x:(i as f32)+500.0, y:(j as f32)}, size: Vector2{x:1.0, y:1.0}}, 
                    c, true, 1.0, false);
            }
        }
        let new_sys_time = SystemTime::now();
        let difference = new_sys_time.duration_since(sys_time)
            .expect("Clock may have gone backwards");
        godot_print!("godot-rust result millisecs: {}", difference.as_millis());
    }

}


fn init(handle: InitHandle) {
    handle.add_class::<HelloWorld>();
}


godot_init!(init);
