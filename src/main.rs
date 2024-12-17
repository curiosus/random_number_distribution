use macroquad::prelude::*;


#[macroquad::main(window_conf)]
async fn main() {

    const TOTAL: usize = 20;
    let mut random_counts: [i32; TOTAL] = [0; TOTAL];

    loop {
        clear_background(BLACK);

        let index = rand::gen_range(0, TOTAL);
        random_counts[index]+=1;

        let w: f32 = screen_width() / random_counts.len() as f32;

        for i in 0..random_counts.len() {
            draw_rectangle(i as f32 * w, screen_height() - random_counts[i] as f32, w - 1.0, random_counts[i] as f32, RED);
        }


        let mut reset: bool = true;
        for count in random_counts {


            if (count as f32) < screen_height() {
                reset = false;
                break;
            } 

        }
        
        if reset {
            random_counts = [0; TOTAL];
        }

        next_frame().await

    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Random Number Distribution".to_owned(),
        window_width: 640,
        window_height: 240,
        fullscreen: false,
        ..Default::default()
    }
}
