use juzhen::Matrix;

fn main() {
    // let m1: Matrix = Matrix::new(3, 2);
    // println!("Output: {m1:?}");

    // let m2: Matrix = Matrix::from_file("src/file.txt");
    // println!("Output: {m2:?}");

    // let mut m3: Matrix = Matrix::from_string("3 6 12 ; 2 3 6 ; 7 6 5");
    // m3.print();
    // m3.apply(|o| o + 2.0);
    // m3.print();

    // let m4: Matrix = Matrix::from_string("1 2 3 ; 4 5 6 ; 7 8 9");
    // m4.print();
    // println!("def(m4) = {:?}", m4.det());

    let m5: Matrix = Matrix::from_string("1 2 4 ; 7 5 -2 ; 3 0 9");
    m5.print();
    
    let m5_1: Matrix = m5.inverse();
    m5_1.print();
}
