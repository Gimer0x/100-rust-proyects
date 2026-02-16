// Day 90: Project: Conway's Game of Life (Terminal Grid Simulation)
// Build a terminal-based simulation of Conway's Game of Life, a cellular automaton 
// that evolves based on simple rules. You'll learn 2D grid modeling, terminal rendering, 
// and game loop logic in Rust.
// Key Concepts:
// + 2D arrays for grid logic
// + Terminal rendering with ANSI escape codes
// + wrapping_add to avoid panics on underflow
// + Game loop + timing with thread::sleep
// You now have a dynamic simulation running in terminal — useful for learning core game 
// dev principles or experimenting with cellular automata.

// Game Rules Recap:
// - Live cell with 2 or 3 neighbors → lives.
// - Dead cell with 3 neighbors → comes alive.
// - All other cells → die or remain dead.
use std::{thread, time::Duration};
 
const WIDTH: usize = 20;
const HEIGHT: usize = 10;
 
type Grid = [[bool; WIDTH]; HEIGHT];
 
fn print_grid(grid: &Grid) {
    print!("\x1B[2J\x1B[1;1H"); // Clear terminal
    for row in grid.iter() {
        for &cell in row.iter() {
            print!("{}", if cell { "⬛" } else { "⬜" });
        }
        println!();
    }
}
 
fn count_neighbors(grid: &Grid, x: usize, y: usize) -> u8 {
    let mut count = 0;
    for dy in [-1, 0, 1] {
        for dx in [-1, 0, 1] {
            if dx == 0 && dy == 0 { continue; }
 
            let nx = x.wrapping_add(dx as usize);
            let ny = y.wrapping_add(dy as usize);
 
            if nx < WIDTH && ny < HEIGHT && grid[ny][nx] {
                count += 1;
            }
        }
    }
    count
}
 
fn step(current: &Grid) -> Grid {
    let mut next = [[false; WIDTH]; HEIGHT];
 
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let alive = current[y][x];
            let neighbors = count_neighbors(current, x, y);
 
            next[y][x] = match (alive, neighbors) {
                (true, 2) | (true, 3) => true,
                (false, 3) => true,
                _ => false,
            };
        }
    }
 
    next
}
 
fn main() {
    println!("Conway's Game of Life - Press Ctrl+C to stop");
    thread::sleep(Duration::from_millis(1000));
 
    let mut grid: Grid = [[false; WIDTH]; HEIGHT];
 
    // Seed: glider pattern
    grid[1][2] = true;
    grid[2][3] = true;
    grid[3][1] = true;
    grid[3][2] = true;
    grid[3][3] = true;
 
    loop {
        print_grid(&grid);
        grid = step(&grid);
        thread::sleep(Duration::from_millis(300));
    }
}