fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut transposed = matrix.clone();
    for i in 0..matrix.len() {
        for j in 0..matrix.len() {
            transposed[i][j] = matrix[j][i];
        }
    }
    transposed
}

fn main() {
    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("matrix: {:#?}", matrix);
    let transposed = transpose(matrix);
    println!("transposed: {:#?}", transposed);
}

#[test]
fn test_transpose() {
    let actual = [
        [1, 2, 3], // <-- the comment makes rustfmt add a newline
        [4, 5, 6],
        [7, 8, 9],
    ];
    let expected = [
        [1, 4, 7], // <-- the comment makes rustfmt add a newline
        [2, 5, 8],
        [3, 6, 9],
    ];

    assert_eq!(transpose(actual), expected);
}
