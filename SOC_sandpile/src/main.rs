use std::{io::{Write}, collections::{VecDeque, HashSet}};
use ndarray::prelude::*;
use rand::prelude::*;
use utilities::save_gnuplot2D;

use crate::utilities::get_file_buffer;

mod utilities;

//grid size
const N: usize = 10+2;
type MatrixInt = Array2<i32>;
type VectorInt = Array1<i32>;


// #### functions ####

fn randomize(Board_in: &mut MatrixInt) {
    let mut Board = Board_in.slice_mut(s![1..-1, 1..-1]);
    let distr = rand::distributions::Uniform::new_inclusive(0, 3);

    for elem in Board.iter_mut() {
        *elem = distr.sample(&mut rand::thread_rng());
    }
}

fn initialize_elem(Board_in: &mut MatrixInt, v: i32) {
    let mut Board = Board_in.slice_mut(s![1..-1, 1..-1]);
    
    for elem in Board.iter_mut() {
        *elem = v;
    }
}

fn initialize_cols(Board_in: &mut MatrixInt, cols: VectorInt, v: i32) {
    let mut Board = Board_in.slice_mut(s![1..-1, 1..-1]);
    
    for col_idx in cols.iter() {
        Board.column_mut(*col_idx as usize).fill(v);
    }
}

/// drops grain of sand at given x,y indices and return number of lattice sites and number of grains taking part in resulting cascade 
/// (if any, otherwise returns 0)
fn drop_and_cascade(idx_tuple: (usize, usize), Board_in: &mut MatrixInt) -> (i32, i32) {
    let (x, y) = idx_tuple;

    let mut FIFO = VecDeque::<(usize, usize)>::new();
    let mut grains_set = HashSet::<(usize, usize)>::new();

    let mut count = 0;

    Board_in[idx_tuple] += 1;

    if Board_in[idx_tuple]  == 4 {
        FIFO.push_front(idx_tuple);
    }
    else {
        return (0, 0);
    }

    while FIFO.len() != 0 {
        count += 4;

        let (x, y) = FIFO.pop_front().unwrap();
        Board_in[(x, y)] = 0;
        // Board_in[(x, y)] -= 4;

        for tuple in array![(x,y+1), (x+1,y), (x,y-1), (x-1,y)] {
            grains_set.insert(tuple);
            Board_in[tuple] += 1;

            if Board_in[tuple] == 4{
                FIFO.push_front(tuple);
            }
        }
        // plot next frame
    }
    (grains_set.len() as i32, count)
    // grains_set.into_iter()
    //             .filter(|(x,y)| (*x != 0 && *x != N-2) && ((*y != 0 && *y != N-2))  )
    //             .count() as i32

}

/// acts the same as drop_and_cascade but also saves board every time cascade occurs
fn drop_and_cascade_save(idx_tuple: (usize, usize), Board_in: &mut MatrixInt, global_count: &mut usize) -> (i32, i32, usize) {
    let (x, y) = idx_tuple;

    let mut FIFO = VecDeque::<(usize, usize)>::new();
    let mut grains_set = HashSet::<(usize, usize)>::new();

    let mut count = 0;

    Board_in[idx_tuple] += 1;

    if Board_in[idx_tuple]  == 4 {
        FIFO.push_front(idx_tuple);
    }
    else {
        return (0, 0, *global_count);
    }

    while FIFO.len() != 0 {
        count += 4;

        let (x, y) = FIFO.pop_front().unwrap();
        Board_in[(x, y)] = 0;
        // Board_in[(x, y)] -= 4;

        for tuple in array![(x,y+1), (x+1,y), (x,y-1), (x-1,y)] {
            grains_set.insert(tuple);
            Board_in[tuple] += 1;

            if Board_in[tuple] == 4{
                FIFO.push_front(tuple);
            }
        }
        // plot next frame
        save_gnuplot2D(&Board_in, &*format!("results/frame_{:02}.txt", *global_count));

        *global_count += 1;
    }
    (grains_set.len() as i32, count, *global_count)
    // grains_set.into_iter()
    //             .filter(|(x,y)| (*x != 0 && *x != N-2) && ((*y != 0 && *y != N-2))  )
    //             .count() as i32
}

fn main() {
    let mut rng = rand::thread_rng();
    let distr = rand::distributions::Uniform::new_inclusive(0, 3);

    
    let mut Board = MatrixInt::from_elem((N,N), 5);
    randomize(&mut Board);

    let uniform_N = rand::distributions::Uniform::new_inclusive(1, N-2);


    //file buffers
    let mut cascade_size_f = get_file_buffer("cascade_size.txt");


    // drops grain at random place on grid (board) and saves every cascade occurence until 8 or more grain cascade occurs 
    let mut global_count: usize = 0;
    for i in 0..1_000 {
        let rand_tuple = (uniform_N.sample(&mut rng),uniform_N.sample(&mut rng));
        
        // let (n, count) = drop_and_cascade(rand_tuple, &mut Board);
        let (n, count, global_count) = drop_and_cascade_save(rand_tuple, &mut Board, &mut global_count);

        // write!(cascade_size_f, "{}\t{}\n", i, count).expect("saving error");
        // println!("{}", i);

        if count >= 8 {
            break;
        }
    }
    


}

