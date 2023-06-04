fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut n = [[0; 3]; 3];
    
    for i in 0..=matrix.len() - 1 {
        for y in 0..=matrix.len() - 1 {
            n[y][i] = matrix[i][y];
        }
    }
    
    n
}

fn pretty_print(matrix: &[[i32; 3]; 3]) {
    for l in matrix {
        println!("{:?}", l);
    }
}

fn main() {
    let matrix = [ // <-- input
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9],
    ];
    
    // [1, 4, 7]   // <-- output
    // [2, 5, 8]
    // [3, 6, 9]

    let transposed = transpose(matrix);
    pretty_print(&transposed);
}

