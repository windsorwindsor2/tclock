use chrono::{Local, Timelike};
use std::f64::consts::PI;
use gemini_engine::{
    core::{ColChar, Modifier, Vec2D},
    gameloop,
    primitives::Line,
    view::{View, WrappingMode},
    ascii::Sprite
};

const FPS: f32 = 4.0;
const VIEW_WIDTH: usize = 80;
const VIEW_HEIGHT: usize = 40;

fn radius(center:Vec2D,r:f64, theta:f64) -> Vec2D{
    //put circle logic here
    let x = 2.0*r*theta.cos();
    let y = r*theta.sin();
    let xcoord = center.x + x as i64;
    let ycoord = center.y + y as i64;
    Vec2D {x:xcoord,y:ycoord}
}

fn main() {

    let mut second_theta:f64 = 0.0;
    let mut minute_theta:f64 = 0.0;
    let mut hour_theta:f64 = 0.0;
    let hour_hand_ratio: f64 = 2.0/3.0;

    let r:f64 = 15.0;

    let center:Vec2D = Vec2D{x:40,y:20};
    let end:Vec2D = radius(center,r,second_theta);
    
    let second_hand_char=ColChar::SOLID.with_mod(Modifier::RED);
    let minute_hand_char=ColChar::SOLID;
    let hour_hand_char=ColChar::SOLID.with_mod(Modifier::CYAN);


    let mut view = View::new(VIEW_WIDTH, VIEW_HEIGHT, ColChar::EMPTY).with_wrapping_mode(WrappingMode::Wrap);

    let mut second_hand = Line::new(center,end, second_hand_char);
    let mut minute_hand = Line::new(center,end, minute_hand_char);
    let mut hour_hand = Line::new(center,end, hour_hand_char);
    let mut second_disp = Sprite::new(Vec2D::ZERO,"00",Modifier::None);

    loop {
        view.clear();

        let now=Local::now();
        let second = now.second() as f64;
        let minute = now.minute() as f64;
        let hour = now.hour() as f64;
        
        let second_text = second.to_string();
        second_theta= (second * PI/30.0) - PI/2.0;
        minute_theta= (minute * PI/30.0) - PI/2.0;
        hour_theta= ((hour + minute/60.0) * PI/6.0) - PI/2.0;

        second_hand.pos1 = radius(center,r,second_theta);
        second_disp.texture=second_text;
        minute_hand.pos1 = radius(center,r,minute_theta);
        hour_hand.pos1= radius(center, r * hour_hand_ratio, hour_theta);

        view.draw(&second_hand);
        view.draw(&second_disp);
        view.draw(&minute_hand);
        view.draw(&hour_hand);

        let _ = view.display_render();
        let _ = gameloop::sleep_fps(FPS, None);
    }
}

