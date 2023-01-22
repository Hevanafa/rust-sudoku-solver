// Sudoku Solver
// by Hevanafa, 21-01-2023

// Adapted from:
// https://stackoverflow.com/questions/19022739/sudoku-solver-in-c

use std::fs::File;
use std::io::{ BufRead, BufReader };

fn find_unassigned(grid: &Vec<Vec<u8>>, row: &mut Box<usize>, col: &mut Box<usize>) -> bool {
    **row = 0;

    loop {
        **col = 0;
        loop {
            if grid[**row][**col] == 0 {
                return true;
            }

            if **col < 8 { **col += 1 }
            else { break }
        }

        if **row < 8 { **row += 1 }
        else { break }
    }

    false
}

fn used_in_row(grid: &Vec<Vec<u8>>, row: usize, num: u8) -> bool {
    for col in 0..9 as usize {
        if grid[row][col] == num { return true }
    }

    false
}

fn used_in_col(grid: &Vec<Vec<u8>>, col: usize, num: u8) -> bool {
    for row in 0..9 as usize {
        if grid[row][col] == num { return true }
    }

    false
}


fn used_in_box(grid: &Vec<Vec<u8>>, box_start_row: usize, box_start_col: usize, num: u8) -> bool {
    for row in 0..3 as usize {
        for col in 0..3 as usize {
            if grid[row + box_start_row][col + box_start_col] == num {
                return true
            }
        }
    }

    false
}

fn is_safe(grid: &Vec<Vec<u8>>, row: usize, col: usize, num: u8) -> bool {
    !used_in_row(grid, row, num) &&
    !used_in_col(grid, col, num) &&
    !used_in_box(grid, row - row % 3, col - col % 3, num)
}


fn solve(grid: &mut Vec<Vec<u8>>) -> bool {
    let mut row: Box<usize> = Box::new(0);
    let mut col: Box<usize> = Box::new(0);

    if !find_unassigned(grid, &mut row, &mut col) {
        return true;
    }

    for num in 1..=9 {
        if is_safe(grid, *row, *col, num) {
            grid[*row][*col] = num;

            if solve(grid) { return true }

            grid[*row][*col] = 0;
        }
    }

    false
}

fn print_grid(grid: &Vec<Vec<u8>>) {
    for row in grid.iter() {
        println!("{}",
            row.iter().map(|cell| cell.to_string()).collect::<Vec<_>>().join(" ")
        );
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut grid: Vec<Vec<u8>> = vec![];

    let f = File::open("sudoku.txt")?;
    let br = BufReader::new(f);

    for l in br.lines() {
        if let Ok(line) = l {
            let mut row: Vec<u8> = line.chars().map(|x|
                if let Some(num) = x.to_digit(10) {
                    num as u8
                } else { 0 }
            ).collect();

            // take care of jagged length
            while row.len() < 9 {
                row.push(0);
            }

            grid.push(row);
        }
    }

    println!("Unsolved:");
    print_grid(&grid);
    println!();

    if solve(&mut grid) {
        println!("Solved:");
        print_grid(&grid);
    } else {
        println!("No solution exists.");
    }

    Ok(())
}
