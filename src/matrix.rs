#[derive(Clone)]
pub struct Vector<K> {
    pub values: Vec<K>,
}

#[derive(Clone)]
pub struct Matrix<K> {
    pub data: Vec<Vector<K>>,
}

pub fn get_matrix_len<K>(m: &Matrix<K>) -> usize {
    m.data.len()
}

pub fn get_vector_len<K>(v: &Vector<K>) -> usize {
    v.values.len()
}

pub fn is_square_matrix<K>(m: &Matrix<K>) -> bool {
    let matrix_len = get_matrix_len(m);
    for fila in &m.data {
        if get_vector_len(fila) != matrix_len {
            return false;
        }
    }
    true
}

pub fn print_vector<K: std::fmt::Display>(v: &Vector<K>) {
    for num in &v.values {
        print!("{} ", num);
    }
    println!();
}

pub fn print_matrix<K: std::fmt::Display>(m: &Matrix<K>) {
    for fila in &m.data {
        print_vector(fila);
    }
}

pub fn vector_to_matrix<K: Clone>(v: &Vector<K>, columns: bool) -> Matrix<K>{
    if columns == true {
        Matrix { data: vec![v.clone()] }
    }
    else {
        Matrix {
        data: v.values.iter()
            .cloned()
            .map(|num| Vector { values: vec![num] })
            .collect()
    }
    }
}

pub fn matrix_to_vector<K: Clone>(m: &Matrix<K>) -> Vector<K> {
    Vector {
        values: m.data.iter()
            .flat_map(|fila| fila.values.iter().cloned())
            .collect()
    }
}

