use proconio::input;

fn main() {
    input!{
        n: i32,
        h_vec: [i32; n],
    }

    // let mut ans = 0;
    let mut n_1 = (h_vec[0] - h_vec[1]).abs();
    let mut n_2 = 0;

    for i in 2..h_vec.len() {
        // println!("前半: {} {}", n_1, n_2);
        let jump1 = n_1 + (h_vec[i] - h_vec[i-1]).abs();
        let jump2 = n_2 + (h_vec[i] - h_vec[i-2]).abs();
        // println!("jump1: {}, jump2: {}", jump1, jump2);
        n_2 = n_1;
        n_1 = if jump1 < jump2 { jump1 } else {jump2};
        // println!("後半: {} {}\n", n_1, n_2);
    }

    println!("{}", n_1);
}