mod matrix;

use matrix::Matrix;
use std::io;
use std::io::Read;
use std::error::Error;
use std::str::FromStr;

fn read_matrix(reader: &mut dyn Read) -> Result<Matrix, Box<dyn Error>> {
    let mut buffer= String::new();
    let read_len = reader.read_to_string(&mut buffer);

    // watafak, there *has* to be a better way to handle this
    if read_len.is_err() {
        return Err(Box::new(read_len.err().unwrap()));
    }

    Matrix::from_str(buffer.as_str())
}

fn main() {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_matrix() {
        let mut text = String::from("4 2\n1 2.0 3 4\n5 6\n7 8");
        let res = read_matrix(&mut text.as_bytes());

        assert!(res.is_ok());

        let mat = res.unwrap();
        assert_eq!(mat.rows, 4);
        assert_eq!(mat.cols, 2);
        assert_eq!(mat[2], [5.0, 6.0]);
    }
}