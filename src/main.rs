use sdl2::{
    event::Event,
    pixels::Color,
    rect::Rect,
    render::{Canvas, RenderTarget},
};

const TILE_SZ: u32 = 64;
const TILES: u32 = 8;
const WINDOW_SZ: u32 = TILES * TILE_SZ;

struct Tile {
    rect: Rect,
    color: Color,
}

impl Tile {
    fn has_match(tile: &Tile) {}
}

fn draw_grid<T: RenderTarget>(canvas: &mut Canvas<T>) {
    for i in 0..TILES {
        let j = (i as u32 * TILE_SZ) as i32 - 1;

        canvas.set_draw_color(Color::RGB(64, 64, 64));
        canvas.draw_line((j, 0), (j, WINDOW_SZ as i32)).expect("");
        canvas.draw_line((0, j), (WINDOW_SZ as i32, j)).expect("");
    }
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("", WINDOW_SZ, WINDOW_SZ)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut events = sdl_context.event_pump().unwrap();

    let mut tiles: Vec<Tile> = Vec::new();

    'game: loop {
        // Clear window
        canvas.set_draw_color(Color::RGB(24, 24, 24));
        canvas.clear();

        // Check events
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } => break 'game,
                Event::MouseButtonDown { x, y, .. } => {
                    let tile = Tile {
                        rect: Rect::new(x / x as i32, y / y as i32, TILE_SZ, TILE_SZ),
                        color: Color::RGB(255, 255, 255),
                    };

                    Tile::has_match(&tile);
                }
                _ => {}
            }
        }

        // Draw grid
        draw_grid(&mut canvas);

        canvas.present();
    }
}
