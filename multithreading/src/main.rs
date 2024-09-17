use std::{thread, time::Instant};

fn main() {
    // Define a large matrix A (100x100 matrix)
    let matrix_a: Vec<Vec<u64>> = (0..100).map(|i| (0..100).map(|j| (i + j) as u64).collect()).collect();

    // Define a large matrix B (100x100 matrix)
    let matrix_b: Vec<Vec<u64>> = (0..100).map(|i| (0..100).map(|j| (i * j) as u64).collect()).collect();

    // Start the timer
    let start = Instant::now();

    // Call the multiply function
    // let result = multiply(matrix_a, matrix_b);
    let result = multiply_parallel(matrix_a, matrix_b);


    // Stop the timer
    let duration = start.elapsed();

    // Print the result
    println!("Result of matrix multiplication:");
    for row in result.iter().take(5) {  // Print only the first 5 rows to avoid flooding the console
        println!("{:?}", row);
    }

    // Print the duration in milliseconds
    println!("Time taken: {} ms", duration.as_millis());
}

fn multiply(matrix_a: Vec<Vec<u64>>, matrix_b: Vec<Vec<u64>>) -> Vec<Vec<u64>> {
    let row_lena = matrix_a.len();
    let col_lena = matrix_a[0].len();
    let row_lenb = matrix_b.len();
    if col_lena != row_lenb {
        panic!("Incompatible multiplication");
    }
    let col_lenb = matrix_b[0].len();
    let mut ans = vec![vec![0; col_lenb]; row_lena];
    for row in 0..row_lena {
        for col in 0..col_lenb {
            for k in 0..col_lena {
                ans[row][col] += matrix_a[row][k] * matrix_b[k][col];
            }
        }
    }
    ans
}

fn multiply_parallel(matrix_a: Vec<Vec<u64>>, matrix_b: Vec<Vec<u64>>) -> Vec<Vec<u64>> {
    let row_lena = matrix_a.len();
    let col_lena = matrix_a[0].len();
    let row_lenb = matrix_b.len();
    if col_lena != row_lenb {
        panic!("Incompatible multiplication");
    }
    let col_lenb = matrix_b[0].len();
    let mut ans = vec![vec![0; col_lenb]; row_lena];
    let mut handles = vec![];
    for row in 0..row_lena {
        let matrix_a_row = matrix_a[row].clone();
        let matrix_b_clone = matrix_b.clone();
        let handle = thread::spawn(move || {
            let mut result_row = vec![0; matrix_b_clone[0].len()];
            for col in 0..matrix_b_clone[0].len() {
                for k in 0..matrix_a_row.len() {
                    result_row[col] += matrix_a_row[k] * matrix_b_clone[k][col];
                }
            }
            result_row
        });
        handles.push(handle);
    }

    // Collect results from all threads
    for (i, handle) in handles.into_iter().enumerate() {
        ans[i] = handle.join().unwrap();
    }
    ans
}
