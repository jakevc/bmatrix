use crossterm::{
    cursor::{Hide, MoveTo, Show},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{self, Clear, ClearType},
    Result,
};
use ctrlc;
use rand::prelude::*;
use scopeguard::defer;
use std::collections::HashSet;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;
use std::time::Duration;

const BITCOIN_SYMBOL: &str = r#"
   000    000  
   000    000
   000    000
00000000000000000000
000000000000000000000  
   000             000 
   000             000 
   000             000 
   000             000 
   000             000 
   000             000 
   00000000000000000
   00000000000000000
   000             000 
   000             000 
   000             000 
   000             000 
   000             000 
   000             000 
000000000000000000000
00000000000000000000  
   000    000 
   000    000 
   000    000
   000    000
"#;

fn is_inside_bitcoin_symbol(
    x: i32,
    y: i32,
    symbol_width: i32,
    symbol_height: i32,
    matrix_width: i32,
    matrix_height: i32,
) -> bool {
    // Adjust these values to position the Bitcoin symbol on the screen
    let offset_x = (matrix_width / 2) - (symbol_width / 2);
    let offset_y = (matrix_height / 2) - (symbol_height / 2);

    x >= offset_x && x < offset_x + symbol_width && y >= offset_y && y < offset_y + symbol_height
}

fn print_green_matrix() -> Result<()> {
    // get termsize to adjust matrix
    let termsize = terminal::size()?;
    let twidth = termsize.0 as i32;
    let theight = termsize.1 as i32;

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl+C handler");

    let matrix_characters: Vec<char> = (33..127).map(|c: u8| c as char).collect();
    let mut rng = rand::thread_rng();
    let symbol_width = twidth / 2 + 24;
    let symbol_height = theight / 2;

    let bitcoin_symbol_coords: HashSet<(i32, i32)> = BITCOIN_SYMBOL
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, ch)| {
                if ch == '0' {
                    Some((x as i32, y as i32))
                } else {
                    None
                }
            })
        })
        .collect();

    execute!(
        std::io::stdout(),
        Hide, // Hide the cursor
        Clear(ClearType::All),
        SetForegroundColor(Color::Green)
    )?;

    defer! {
        execute!(std::io::stdout(), Show, ResetColor).unwrap();
        terminal::disable_raw_mode().unwrap();
    }

    loop {
        for row in 0..theight {
            for col in 0..twidth {
                if !is_inside_bitcoin_symbol(col, row, symbol_width, symbol_height, twidth, theight)
                    || !bitcoin_symbol_coords.contains(&(
                        col - (twidth / 2 - symbol_width / 2),
                        row - (theight / 2 - symbol_height / 2),
                    ))
                {
                    let random_char = matrix_characters[rng.gen_range(0..matrix_characters.len())];
                    execute!(
                        std::io::stdout(),
                        MoveTo(col as u16 * 2, row as u16),
                        Print(random_char)
                    )?;
                } else {
                    execute!(
                        std::io::stdout(),
                        MoveTo(col as u16 * 2, row as u16),
                        Print(' ')
                    )?;
                }
            }
        }

        // Adjust the sleep duration to control the matrix update speed
        thread::sleep(Duration::from_millis(200));
    }
}

fn main() -> Result<()> {
    terminal::enable_raw_mode()?;
    print_green_matrix()
}
