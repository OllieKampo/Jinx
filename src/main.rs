pub mod moremath;
pub mod common;

use moremath::matrix::Matrix;
use std::time::Instant;
use rand;

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    let rows = 100;
    let cols = 100;
    let mut matrix1 = Matrix::new(rows, cols);
    let mut matrix2 = Matrix::new(rows, cols);

    // Initialize matrices with some values
    for i in 0..rows {
        for j in 0..cols {
            matrix1.set(i, j, (i as i64) as f64);
            matrix2.set(i, j, 1.0);
        }
    }

    let times = 1000;
    let mut total_duration = 0;
    let mut sum = 0.0;
    for _ in 0..times {
        for i in 0..rows {
            for j in 0..cols {
                matrix1.set(i, j, rand::random::<f64>());
            }
        }

        let start = Instant::now();
        let _result = &matrix1 + &matrix2;
        let duration = start.elapsed();
        total_duration += duration.as_nanos();
        for i in 0..rows {
            for j in 0..cols {
                sum += _result.get(i, j);
            }
        }
    }
    println!("{}", sum);

    // let result = &matrix1 + &matrix2;
    // print!("{}", result);

    let average_duration = total_duration / times;
    println!("Total time taken to add two matrices {} times: {} seconds", times, total_duration as f64 / 1000000000.0);
    println!("Average time taken to add two matrices: {} nanoseconds", average_duration);

    let mut raw_matrix_array_1: [f64; 10000] = [0.0; 10000];
    let mut raw_matrix_array_2: [f64; 10000] = [0.0; 10000];
    for i in 0..10000 {
        raw_matrix_array_1[i] = (i as i64) as f64;
        raw_matrix_array_2[i] = 1.0;
    }

    let mut total_duration = 0;
    let mut sum = 0.0;
    for _ in 0..times {
        for i in 0..10000 {
            raw_matrix_array_1[i] = rand::random::<f64>();
        }

        let start = Instant::now();
        let mut raw_result: [f64; 10000] = [0.0; 10000];
        for i in 0..10000 {
            raw_result[i] = raw_matrix_array_1[i] + raw_matrix_array_2[i];
        }
        let duration = start.elapsed();
        total_duration += duration.as_nanos();
        sum += raw_result.iter().sum::<f64>();
    }
    println!("{}", sum);

    let average_duration = total_duration / times;
    println!("Total time taken to add two arrays {} times: {} seconds", times, total_duration as f64 / 1e9);
    println!("Average time taken to add two arrays: {} nanoseconds", average_duration);
}
