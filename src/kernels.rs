use crate::board::Board;
use crate::globals::{NUM_COLS, NUM_ROWS};

/// Updates a given board previous_board to in the next_board

#[derive(Debug)]
pub enum Kernels {
    CpuSequential
}

impl Kernels {
    pub(crate) fn clone(&self) -> Kernels {
        match self {
            Kernels::CpuSequential => Kernels::CpuSequential
        }
    }
}

pub fn get_kernel_func(kernel: Kernels) -> fn(&mut Board, &mut Board) {
    match kernel {
        Kernels::CpuSequential => update_board
    }
}

pub fn update_board(previous_board: &mut Board, next_board: &mut Board){
    // Inner board computation as the edges are off
    for ix in 1..*NUM_COLS.read().unwrap() as usize -1 {
        for iy in 1..*NUM_ROWS.read().unwrap() as usize -1 {
            let total_neighbors =
                    previous_board[ix][iy-1] +
                    previous_board[ix][iy+1] +
                    previous_board[ix-1][iy] +
                    previous_board[ix+1][iy] +
                    previous_board[ix+1][iy-1] +
                    previous_board[ix-1][iy+1] +
                    previous_board[ix-1][iy-1] +
                    previous_board[ix+1][iy+1];

            if previous_board[ix][iy] == 1 {
                next_board[ix][iy] = match total_neighbors {
                    3 => {1}
                    2 => {1}
                    _ => {0}
                }
            }
            else {
                next_board[ix][iy] = match total_neighbors {
                    3 => {1}
                    _ => {0}
                }
            }
        }
    }
}
