use juzhen::Matrix;

 fn main() {
    let m1: Matrix = Matrix::new(3, 2);
    println!("Output: {m1:?}");

    let m2: Matrix = Matrix::from_file("src/file.txt");
    println!("Output: {m2:?}");

    let mut m3: Matrix = Matrix::from_string("3 6 12 ; 2 3 6 ; 7 6 5");
    m3.print();
    m3.apply(|o| o+2.0);
    m3.print();
}