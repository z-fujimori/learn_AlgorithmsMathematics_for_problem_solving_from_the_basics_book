use proconio::input;

fn main() {
    input!{
        n: i32,
        // h_vec: [i32; n]
    }

    let mut n_2 = 1;
    let mut n_1 = 1;

    for _i in 2..=n {
        let pool = n_1;
        n_1 = n_1 + n_2;
        n_2 = pool;
    }


    println!("{}", n_1);
}