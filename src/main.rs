use raylib::{prelude::*};

struct Region {
    id: u32,
    x: i32,
    y: i32,
    size: i64,
    troops: u32,
    color: Color,
    is_hq: bool
}

fn main() {
    let (mut rl, thread) = raylib::init()
    .size(800, 600)
    .title("Hello, World")
    .build();

    rl.set_target_fps(60);

    let mut regions: Vec<Region> = Vec::new();

    let tile_size = 50;    

   for col in 1..14{
    for row in 1..10{
        let pos_x = col * tile_size;
        let pos_y = row * tile_size;
        
        regions.push(Region{
        id: (col * 14 + row) as u32,
        x: pos_x,
        y: pos_y,
        size: tile_size as i64,
        color: Color::BLACK,
        troops: 0,
        is_hq: false
        });

    }
    }
    for region in &mut regions {
    if region.x == 1 * tile_size && region.y == 5 * tile_size {
        region.color = Color::RED;
        region.troops = 5;
        region.is_hq = true;
    }
    if region.x == 13 * tile_size && region.y == 5 * tile_size {
        region.color = Color::BLUE;
        region.troops = 5;
        region.is_hq = true;
    }
}


    

    let mut game_over:bool = false;
    let mut game_start:bool = false;
    let mut is_red_turn:bool = true;
    let mut selected_region_id: Option<u32> = None;
    let mut move_amount = 0;
    

    while !rl.window_should_close() {
    let mut d = rl.begin_drawing(&thread);
    
    d.clear_background(Color::YELLOWGREEN);

    let mouse_pos = d.get_mouse_position();

    let mut red_regions = 0;
    let mut blue_regions = 0;
    


    let mouse_x= mouse_pos.x as i32;
    let mouse_y= mouse_pos.y as i32;

    let left_mouse_clicked: bool = d.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT);
    let r_button_clicked: bool = d.is_key_pressed(KeyboardKey::KEY_R);
    
    if let Some(key) = d.get_key_pressed() {
                    match key {
                    KeyboardKey::KEY_ONE => move_amount = 1,
                    KeyboardKey::KEY_TWO => move_amount = 2,
                    KeyboardKey::KEY_THREE => move_amount = 3,
                    KeyboardKey::KEY_FOUR => move_amount = 4,
                    KeyboardKey::KEY_FIVE => move_amount = 5,
                    _ => {} 
                    }
                }


    let mut move_action: Option<(u32, u32)> = None;            

    for region in &mut regions{
        if mouse_x >= region.x 
        && mouse_x < region.x + region.size as i32
        && mouse_y >= region.y 
        && mouse_y < region.y + region.size as i32{
           if left_mouse_clicked == true && game_over == false{
            if is_red_turn == true{
                match selected_region_id {
                None => {
                    if region.color == Color::RED && region.troops > 0{  
                    selected_region_id = Some(region.id);
                    }
                }
                Some(source_id) => {
                 if region.color == Color::BLACK{

                    move_action = Some((source_id, region.id));

                    selected_region_id = None;
                 }
                }
                }

            }
            else if is_red_turn == false{
                match selected_region_id {
                None => {
                    if region.color == Color::BLUE && region.troops > 0{  
                    d.draw_text("How many Units?", region.x, region.y, 15, Color::WHITE);
                    selected_region_id = Some(region.id);
                    }
                }
                Some(source_id) => {
                 if move_amount != 0 && region.color == Color::BLACK{

                    move_action = Some((source_id, region.id));

                    selected_region_id = None;
                 }
                }
                }
            }
        }        
    }        
            
    }    

   if let Some((from_id, to_id)) = move_action {
    let mut valid_move = false;

    for region in &mut regions {
        if region.id == from_id {
            if region.troops >= move_amount && move_amount > 0 {
                region.troops -= move_amount;
                valid_move = true;
            }
        }
    }

    if valid_move {
        for region in &mut regions {
            if region.id == to_id {
                region.troops += move_amount;
                region.color = if is_red_turn { Color::RED } else { Color::BLUE };
            }
    
        }   
        is_red_turn = !is_red_turn;    
    }        
            
    }
    if r_button_clicked {
    game_over = false;
    game_start = false;
    for region in &mut regions {
        region.color = Color::BLACK;
        if region.x == 1 * tile_size && region.y == 5 * tile_size {
        region.color = Color::RED;
        region.troops = 5;
        region.is_hq = true;
    }
    if region.x == 13 * tile_size && region.y == 5 * tile_size {
        region.color = Color::BLUE;
        region.troops = 5;
        region.is_hq = true;
    }
        
    }
    }
   

    for region in &regions {
        if region.color == Color::RED {
        red_regions += 1;
        } else if region.color == Color::BLUE {
        blue_regions += 1;
        }
    }

   




    for region in &regions {
        d.draw_rectangle(
        region.x,
        region.y,
        region.size as i32,
        region.size as i32,
        region.color,
        );

        d.draw_rectangle_lines(
        region.x,
        region.y,
        region.size as i32,
        region.size as i32,
        Color::WHITE, // Contrast color for the grid lines

    );

        if region.troops > 0{
            let unit_text = format!("{}", region.troops);
            d.draw_text(&unit_text, region.x + 18, region.y + 12, 24, Color::WHITE);
        }

        if let Some(selected_id) = selected_region_id {
            if region.id == selected_id {
                d.draw_rectangle_lines_ex(
                    Rectangle {
                        x: region.x as f32,
                        y: region.y as f32,
                        width: region.size as f32,
                        height: region.size as f32,
                    },
                        3.0,
                        Color::YELLOW,
                );
                d.draw_text("How many Units?", region.x, region.y, 15, Color::WHITE);
            }
        }
    }   

    let red_text = format!("Red: {}", red_regions);
    let blue_text = format!("Blue: {}", blue_regions);

    d.draw_text(&red_text, 20, 500, 25, Color::RED);
    d.draw_text(&blue_text, 20, 545, 25, Color::BLUE);
    
    let move_text = format!("Move Amount: {} Press 1-5 to Change", move_amount);
    d.draw_text(&move_text, 20, 460, 20, Color::WHITE);

    if blue_regions > 0 && red_regions > 0{
        game_start = true;
    }

   

    if blue_regions > 0  && red_regions == 0 && game_start == true{
        d.draw_text("Blue Victory!", 0,0,100,Color::BLUE);
        game_over = true;
    }
    if red_regions > 0 && blue_regions == 0 && game_start == true{
        d.draw_text("Red Victory!", 0,0,100,Color::RED);
        game_over = true;
    }

}
}  

