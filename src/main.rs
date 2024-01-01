use juzhen::Matrix;

 fn main() {
    let m1: Matrix = Matrix::new(3, 2);
    println!("Output: {m1:?}");

    let m2: Matrix = Matrix::from_file("src/file.txt");
    println!("Output: {m2:?}");

    let m3: Matrix = Matrix::from_string("3, 6, 12 ; 2, 3, 6 ; 7 6 5");
    println!("Output: {m3:?}");
}