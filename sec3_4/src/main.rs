use std::cmp::max;

use proconio::input;

fn main() {
    input!{
        n: usize,
        w: usize,
        vw: [[i64; 2]; n]
    }

    // println!("{:?} {} {}", vw, n, w);

    let mut back = vec![vec![0; w+1]; n+1];

    for i in 1..=n {
        for j in 0..=w {
            if j < vw[i-1][0] as usize {
                back[i][j] = back[i-1][j];
            } else {
                // println!("{} {}",back[i-1][j], back[i-1][j-vw[i-1][0] as usize] + vw[i-1][1]);
                back[i][j] = max(back[i-1][j], back[i-1][j-vw[i-1][0] as usize] + vw[i-1][1]);
            }
        }
    }


    println!("{}", back[n][w]);
}