use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{WindowCanvas, Texture};
use sdl2::rect::{Point, Rect};
use sdl2::image::{self, LoadTexture, InitFlag};
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Debug)]
struct Player {
    position: Point,
    sprite: Rect,
    speed: i32,
    speed_co: i32,
    direction: Direction,
    current_frame: i32,
}

fn sprite_picker(direction: Direction) -> i32 {
    use self::Direction::*;
    match direction {
        Down => 0,
        Left => 1,
        Right => 2,
        Up => 3,
    }
}

fn render(
    canvas: &mut WindowCanvas,
    color: Color,
    texture: &Texture,
    player: &Player,
) -> Result<(), String> {
    canvas.set_draw_color(color);
    canvas.clear();

    let (width, height) = canvas.output_size()?;
    let (frame_width, frame_height) = player.sprite.size();

    let current_sprite = Rect::new(
        player.sprite.x() + frame_width as i32 * player.current_frame,
        player.sprite.y() + frame_height as  i32 * sprite_picker(player.direction),
        frame_width,
        frame_height,
    );

    let screen_position = player.position + Point::new(width as i32/2, height as i32/2);
    let screen_rect = Rect::from_center(screen_position, frame_width, frame_height);
    canvas.copy(texture, current_sprite, screen_rect)?;
    canvas.present();

    Ok(())
}

fn update_player(player: &mut Player) {
    use self::Direction::*;
    match player.direction {
        Left => {
            player.position = player.position.offset(-player.speed, 0);
        },
        Right => {
            player.position = player.position.offset(player.speed, 0);
        },
        Down => {
            player.position = player.position.offset(0, player.speed);
        },
        Up => {
            player.position = player.position.offset(0, -player.speed);
        },
    }
    if player.speed != 0{
        player.current_frame = (player.current_frame+1) % 3;
    }
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;

    let window = video_subsystem
        .window("simple_game", 800, 600)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("could not create canvas");

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture("assets/bardo.png")?;
    let mut player = Player {
        position: Point::new(0,0),
        sprite: Rect::new(0,0,26,36),
        speed: 0,
        speed_co: 5,
        direction: Direction::Down,
        current_frame: 0
    };

    let mut event_pump = sdl_context.event_pump()?;
    let mut i = 0;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                Event::KeyDown {keycode: Some(Keycode::Left), repeat: false, ..} | Event::KeyDown {keycode: Some(Keycode::A), repeat: false, ..} => {
                    player.speed = player.speed_co;
                    player.direction = Direction::Left;
                },
                Event::KeyDown {keycode: Some(Keycode::Right), repeat: false, ..} | Event::KeyDown {keycode: Some(Keycode::D), repeat: false, ..} => {
                    player.speed = player.speed_co;
                    player.direction = Direction::Right;
                },
                Event::KeyDown {keycode: Some(Keycode::Down), repeat: false, ..} | Event::KeyDown {keycode: Some(Keycode::S), repeat: false, ..} => {
                    player.speed = player.speed_co;
                    player.direction = Direction::Down;
                },
                Event::KeyDown {keycode: Some(Keycode::Up), repeat: false, ..} | Event::KeyDown {keycode: Some(Keycode::W), repeat: false, ..} => {
                    player.speed = player.speed_co;
                    player.direction = Direction::Up;
                },
                Event::KeyDown {keycode: Some(Keycode::Space), repeat: false, ..} => {
                    player.speed_co = 10;
                },
                Event::KeyUp {keycode: Some(Keycode::Space), repeat: false, ..} => {
                    player.speed_co = 5;
                }
                Event::KeyUp { keycode: Some(Keycode::Left), repeat: false, ..} |
                Event::KeyUp { keycode: Some(Keycode::Right), repeat: false, ..} |
                Event::KeyUp { keycode: Some(Keycode::Down), repeat: false, ..} |
                Event::KeyUp { keycode: Some(Keycode::Up), repeat: false, ..} |
                Event::KeyUp { keycode: Some(Keycode::A), repeat: false, ..} |
                Event::KeyUp { keycode: Some(Keycode::S), repeat: false, ..} |
                Event::KeyUp { keycode: Some(Keycode::D), repeat: false, ..} |
                Event::KeyUp { keycode: Some(Keycode::W), repeat: false, ..} => {
                    player.speed = 0;
                }
                _ => {}
            }
        }
        i = (i + 1) % 255;
        // player.dash
        //...
        // if player.speed_co > 5 drain 1 stam. If stam == 0 player.speed_co = 5 
        update_player(&mut player);
        render(&mut canvas, Color::RGB(i, i/2, 255 - i), &texture, &player)?;
        // The rest of the game loop goes here...
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}

