// Uses the ndarray crate
use ndarray::{arr1, arr2, Array1};

fn main() {
    let scalar = 8;
    let vector = arr1(&[0, 1, 2]);
    let array1 = arr2(&[[1, 2, 3],
                                       [4, 5, 6]]);
    let array2 = arr2(&[[4, 3, 1,],
                                       [7, 8, 6]]);
    let array3 = arr2(&[[2, 3],
                                       [6, 9],
                                       [1, 5]]);

    let sum = &array1 + &array2;

    // Matrix multiplication is performed with ndarray::ArrayBase::dot
    let multiplication = array1.dot(&array3);

    // Multiply a scalar with a vector with a matrix
    let new_vector: Array1<_> = scalar * vector;
    let new_matrix = array1.dot(&new_vector);

    println!("===== Sum =====");
    println!("{}", array1);
    println!("+");
    println!("{}", array2);
    println!("=");
    println!("{}", sum);

    println!("===== Multiplication =====");
    println!("{}", array1);
    println!("*");
    println!("{}", array3);
    println!("=");
    println!("{}", multiplication);

    println!("New Vector: {}", new_vector);
    println!("New Matrix: {}", new_matrix);


}