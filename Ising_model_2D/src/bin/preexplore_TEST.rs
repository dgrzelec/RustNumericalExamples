use preexplorer::prelude::*;
use ndarray::prelude::*;


const N: usize = 10;

fn main() {
    // Simpliest use

    let data = 0..10;
    data.preexplore().save_with_id("id_test").unwrap();

    let data_vector = array![[2,3,4,5,6,7,8,9]];
    data_vector.preexplore()
        .set_title("Linear")
        .set_labelx("X")
        .set_labely("Y")
        .plot("ndarray_vector").unwrap();

    // data_vector.preexplore().plot_later("nalgebra_vector").unwrap();

    let mut data2D = Array2::<i32>::ones((N,N));
    // brzegi zerowe
    data2D.row_mut(0).fill(0);
    data2D.row_mut(N-1).fill(0);
    data2D.column_mut(0).fill(0);
    data2D.column_mut(N-1).fill(0);
    
    for row in data2D.rows() {

        row.preexplore().save_with_id("ndarray_brzegi").unwrap();
    }


    // pre::Heatmap::from(data2D)
    //     .set_title("testowa heatmapa")
    //     .plot("heatmap_test").unwrap();
}