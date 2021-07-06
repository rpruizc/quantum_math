// Uses the ndarray crate
use ndarray::arr2;

fn main() {
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

}