mod matrix;

use matrix::{Matrix, Vector, is_square_matrix, print_matrix, get_matrix_len};

fn main() {
    let v1 = Vector {values: vec![1, 2, 3]};
    let v2 = Vector {values: vec![4, 5, 6]};
    let v3 = Vector {values: vec![7, 8, 9]};

    let m1 = Matrix {data: vec![v1, v2.clone(), v3]};

    println!("El valor es: {}", m1.data[0].values[2].to_string());
    println!("El tamaño de la Matrix es: {}", get_matrix_len(&m1).to_string());

    print_matrix(&m1);
    if is_square_matrix(&m1) {
        println!("Es cuadrada");
    }
    else {
        println!("No es cuadrada");
    }
}
