use std::{io::{BufWriter, Write}, fs::File};
use ndarray::prelude::*;
use rand::prelude::*;
use utilities::save_gnuplot2D;

use crate::utilities::get_file_buffer;

mod utilities;

//grid size
const N: usize = 16+1;
type MatrixInt = Array2<i32>;
type VectorInt = Array1<i32>;

fn randomize(board: &mut MatrixInt, p: f64) {
    let distr = rand::distributions::Uniform::new_inclusive(0.0, 1.0);

    for elem in board.iter_mut(){
        if thread_rng().sample(distr) < p {
            *elem = 1;
        }
        else {
            *elem = 0;
        }
    }
}

fn find(x: i32, Labels: &mut VectorInt) -> i32 {
    let mut y = x;
    while Labels[y as usize] != y {
        y = Labels[y as usize];
    }
    while Labels[x as usize] != x {
        let mut z = Labels[x as usize];
        Labels[x as usize] = y;
        let mut x = z;
    }
    y
}

fn union(xa: i32, ya: i32, Labels: &mut VectorInt) {
    // copied find for xa
    let mut y = xa;
    let mut x = xa;
    while Labels[y as usize] != y {
        y = Labels[y as usize];
    }
    while Labels[x as usize] != x {
        let mut z = Labels[x as usize];
        Labels[x as usize] = y;
        let mut x = z;
    }
    let xa = y;
    
    // copied find for ya
    let mut y = ya;
    let mut x = ya;
    while Labels[y as usize] != y {
        y = Labels[y as usize];
    }
    while Labels[x as usize] != x {
        let mut z = Labels[x as usize];
        Labels[x as usize] = y;
        let mut x = z;
    }
    let ya = y;
    

    Labels[xa as usize ] = ya;
}

fn find_cluster(x: i32, Labels: &Array1<i32>) -> i32 {
    let mut y = x;
    while Labels[y as usize] != y {
        y = Labels[y as usize];
    }
    y
}

fn HK_algorithm(board: &MatrixInt, Label: &mut MatrixInt) {

    let mut Labels = VectorInt::zeros(N*N);
    for i in 0..N*N {
        Labels[i] = i as i32;
    }

    let mut largest = 0;
    Label.fill(0);

    for x in 1..N {
        for y in 1..N {

            if board[[x,y]] == 1 {
                let (left, down) = (Label[[ (x-1), y ]], Label[[ x, (y-1) ]]);

                match (left, down) {
                    (0,0) => {largest += 1;
                             Label[[x,y]] = largest;},
                    (_,0) => Label[[x, y]] = find_cluster(left, &mut Labels),
                    (0,_) => Label[[x, y]] = find_cluster(down, &mut Labels),
                    (_,_) => {//link left and above clusters
                             let temp_left = find_cluster(left, &mut Labels);
                             Labels[ temp_left as usize] = find_cluster(down, &mut Labels);
                             Label[[x, y]] = find_cluster(left, &mut Labels);
                                },
                }



            }
        }
    }

    // additional loop to correctly map cluster labels
    for elem in Label.iter_mut(){
        *elem = find_cluster(*elem, &Labels);
    }

}

fn check_percolation(Label: &MatrixInt) -> bool {
    let first_col_iter = Label.column(1).clone().into_iter();
    let last_col_iter = Label.column(N-1).clone().into_iter();

    let first_col = Vec::from_iter(first_col_iter);
    let last_col = Vec::from_iter(last_col_iter);

    first_col.iter().any(|item| if **item != 0 && last_col.contains(item){ return true} else { return false})
    
}

fn count_cluster_index(idx: i32, Label: &MatrixInt) -> i32 {
    Label.iter().filter(|&x| *x == idx).count() as i32
}


fn main() {
    let mut rng = rand::thread_rng();
    let distr = rand::distributions::Uniform::new_inclusive(0.0, 1.0);

    
    let mut Board = MatrixInt::from_elem((N,N), 1);
    let mut Label = MatrixInt::zeros((N,N));


    let p = 0.6;
    randomize(&mut Board, p);

    HK_algorithm(&mut Board, &mut Label);

    // save_gnuplot2D(&Label, "test_100x100.txt");

    check_percolation(&Label);


    // percolation probability dependance on parameter p, which is a probability of filling the cell (prob. of occupancy)

    let Nsteps: usize = 100;

    let path = &*format!("p_plot_L{}.txt", N);
    let mut f = get_file_buffer(path);

    for i in 1..=100 {
        let p = 0.01*(i as f64);
        println!("{}", p);

        let mut stat = 0.0;
        for n in 0..Nsteps {
            randomize(&mut Board, p);
            
            HK_algorithm(&mut Board, &mut Label);
    
            if check_percolation(&Label){
                stat += 1.;
            }
        }

        write!(f, "{} {}\n", p, stat/(Nsteps as f64)).expect("error during saving to file");
    }



}
