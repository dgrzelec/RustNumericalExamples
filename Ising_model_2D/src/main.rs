use std::{io::{BufWriter, Write}, fs::File};

use ndarray::prelude::*;
use num::traits::{ops::inv, Pow};
use preexplorer::prelude::*;
use rand::prelude::*;

use crate::utilities::get_file_buffer;

mod utilities;

//grid size
const N: usize = 500;
const N2: usize = 250_000;
const dE_possibilities: usize = 5;

type MatrixInt = Array2<i32>;


// parameters
const J: f64 = 1.0;
const k_b: f64 = 1.0;
// only 5 values of dE can appear in 4-neighbours model
const dE_array: [i32; dE_possibilities] = [-8, -4, 0, 4 ,8];

/// generate array of probabilities based on possible values of dE from dE_array
fn gen_transition_probabilities(invT: f64) -> [f64; dE_possibilities]{

    dE_array.map(|p| if (-(p as f64)*invT).exp() < 1. { return (-(p as f64)*invT).exp()} else { return 1.})
}
/// calculate magnetization of a whole board as a sum of spin values (-1 or 1)
fn magnetization(board: &MatrixInt) -> f64 {
    let ret: f64 = (board.sum()).into();
    ret
}
// unused
fn magnetization2(board: &MatrixInt) -> (f64, f64) {
    let m: f64 = (board.sum()).into();
    let m2: f64 = N2 as f64;
    (m, m2)
}
/// Calculate energy of a whole board
fn energy(board: &MatrixInt) -> f64 {
    let mut E = 0;
    for ((i,j), spin) in board.indexed_iter() {
        E += -spin*(board[[(N+i-1)%N,j]] + board[[(N+i+1)%N ,j]] + board[[i,(N+j-1)%N]] + board[[i, (N+j+1)%N ]] )
    }
    (E as f64)/(N2 as f64)
}
/// Calculate energy and energy squared of a whole board
fn energy2(board: &MatrixInt) -> (f64, f64) {
    let mut E = 0;
    let mut E2 = 0;
    for ((i,j), spin) in board.indexed_iter() {
        E += -spin*(board[[(N+i-1)%N,j]] + 
                    board[[(N+i+1)%N ,j]] + 
                    board[[i,(N+j-1)%N]] + 
                    board[[i, (N+j+1)%N ]] );
        E2 += E.pow(2);
    }
    ((E as f64)/(N2 as f64), (E2 as f64)/(N2 as f64))
}
/// do one monte carlo iteration step; loops through the board in random order and tries to flip every spin 
fn iterate(board: &mut MatrixInt, prob_tab: &[f64;dE_possibilities], x_indices: &mut Vec<usize>, y_indices: &mut Vec<usize>) {

    let distr = rand::distributions::Uniform::new_inclusive(0.0, 1.0);

    x_indices.shuffle(&mut thread_rng());
    y_indices.shuffle(&mut thread_rng());
    let mut dE = 0;

    'x_loop: for &x_i in x_indices.iter() {
        'y_loop: for &y_i in y_indices.iter() {

            dE = 2*board[[x_i,y_i]] * (board[[(N+x_i-1)%N,y_i]] + 
                    board[[(N+x_i+1)%N ,y_i]] + 
                    board[[x_i,(N+y_i-1)%N]] + 
                    board[[x_i, (N+y_i+1)%N ]] );
            
                    // choosing the right probability 
                    let mut idx_temp = dE_array.iter().position(|&x| x == dE).unwrap();
                    let p = prob_tab[idx_temp];
                    // change spin with probability p
                    if thread_rng().sample(distr) < p {
                        board[[x_i,y_i]] *= -1;
                    }

        }       
    }

}

fn randomize(board: &mut MatrixInt) {
    let distr = rand::distributions::Uniform::new_inclusive(0.0, 1.0);

    for elem in board.iter_mut(){
        if thread_rng().sample(distr) < 0.5 {
            *elem *= -1;
        }
    }
}
/// prints the board as up/down arrows
fn render(board: &MatrixInt) {
    for row in board.rows() {
        for &spin in row {
            match spin {
                -1 => print!(" ↓ "),
                1 => print!(" ↑ "),
                _ => print!(" 0 "),
            }
        }
        println!();
    }
}
/// saves the board as up/down arrows to given file buffer
fn render_save(board: &MatrixInt, f: &mut BufWriter<File>) {
    for row in board.rows() {
        for &spin in row {
            match spin {
                -1 => write!(f, " ↓ ").expect("nie udało sie zapisac"),
                1 => write!(f," ↑ ").expect("nie udało sie zapisac"),
                _ => write!(f," 0 ").expect("nie udało sie zapisac"),
            }
        }
        write!(f, "\n").expect("nie udało sie zapisac");
    }
}

fn main() {

    let mut rng = rand::thread_rng();
    let distr = rand::distributions::Uniform::new_inclusive(0.0, 1.0);



let mut Board = MatrixInt::from_elem((N,N), 1);

// these will be shuffled
let mut x_indices: Vec<usize> = (0..N).collect();
let mut y_indices: Vec<usize> = (0..N).collect();

// println!("Magnetization={};  Energy={};", magnetization(&Board), energy(&Board));

// ########## 30 point in temperature ########
let tau: usize = 10_000;
let mut f_zad4 = get_file_buffer("temperature.txt");


for Ti in 0..30 {
    let mut T = 1.5 + (Ti as f64)*0.05;
    let mut invT = 1./T;
    
    let mut M_sum = 0.0;
    let mut M2_sum = 0.0;
    
    let mut Board = MatrixInt::from_elem((N,N), 1);
    // randomize(&mut Board);

    let mut p_tab = gen_transition_probabilities(invT);
    
    for mcs_i in 0..(90_000) {

        iterate(&mut Board, &p_tab, &mut x_indices, &mut y_indices);
    }

    for mcs_i in 0..tau {
        iterate(&mut Board, &p_tab, &mut x_indices, &mut y_indices);
        let m = magnetization(&Board);
        M_sum += m;
        M2_sum += m.pow(2);
    }

    write!(f_zad4, "{:<5.6} {:<5.6} {:<5.6}\n", T, M_sum/(tau as f64), M2_sum/(tau as f64)).expect("error during saving");
}




// #########################################################

// E_tab.preexplore().set_labelx("mcs step").set_labely("E").plot_later("after10k_beta_05").unwrap();


// ############# GIF FRAMES ##############
// generate 100 gif frames and save as separate .txt files
randomize(&mut Board);

let mut invT: f64 = 0.3;
let mut p_tab = gen_transition_probabilities(invT);

let Nmcs: usize = 100;

for mcs_i in 0..Nmcs {

    iterate(&mut Board, &p_tab, &mut x_indices, &mut y_indices);
    
    // if mcs_i%10 == 0 {
        print!("step: {}", mcs_i);

        let mut fname = format!("frames/{:04}.txt", mcs_i);
        utilities::save_gnuplot2D(&Board, &*fname);
    // }

}


// #######################################

}