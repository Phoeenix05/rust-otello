use sdl2::{
    event::Event,
    pixels::Color,
    rect::{Point, Rect},
    render::{Canvas, RenderTarget},
};

const TILE_SIZE: u32 = 64;
const TILES: usize = 8;
const WINDOW_SIZE: u32 = (TILES as i32 * TILE_SIZE as i32) as u32;

fn draw_grid<T: RenderTarget>(grid: &[[i32; TILES]; TILES], canvas: &mut Canvas<T>) {
    let mut x: i32 = -1;
    let mut y: i32 = -1;

    canvas.set_draw_color(Color::RGB(255, 255, 255));

    for _ in 0..TILES {
        canvas
            .draw_line(Point::new(x, 0), Point::new(y, WINDOW_SIZE as i32))
            .expect("");
        canvas
            .draw_line(Point::new(0, y), Point::new(WINDOW_SIZE as i32, x))
            .expect("");

        x = x + TILE_SIZE as i32;
        y = y + TILE_SIZE as i32;
    }

    for y in grid[0] {
        for x in grid[y as usize] {
            if grid[y as usize][x as usize] == 1 {
                canvas
                    .fill_rect(Rect::new(
                        x * TILE_SIZE as i32,
                        y * TILE_SIZE as i32,
                        TILE_SIZE,
                        TILE_SIZE
                    ))
                    .expect("");
            }
        }
    }
}

fn mouse_pos_to_grid(p: [i32; 2]) -> [usize; 2] {
    [
        (p[0] / TILE_SIZE as i32) as usize,
        (p[1] / TILE_SIZE as i32) as usize,
    ]
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("", WINDOW_SIZE, WINDOW_SIZE)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut events = sdl_context.event_pump().unwrap();

    let mut grid = [[0i32; TILES as usize]; TILES as usize];

    'game: loop {
        // Clear window
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        // Check events
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } => break 'game,
                Event::MouseButtonDown { x, y, .. } => {
                    let mpos = mouse_pos_to_grid([x, y]);
                    println!("{:?}", mpos);
                    grid[mpos[1]][mpos[0]] = 1;
                }
                _ => {}
            }
        }

        // Draw grid
        draw_grid(&grid, &mut canvas);

        canvas.present();
    }
}
