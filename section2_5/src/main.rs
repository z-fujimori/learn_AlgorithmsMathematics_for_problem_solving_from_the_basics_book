use proconio::input;

fn main() {
    input! {
        n: i32
    }

    // let mut result = vec![];
    let mut result: Vec<i32> = (2..=n).collect();
    let mut frag: i32 = 2;

    // println!("{:?}",result);

    while frag < result.len() as i32 {
        result = result.iter().filter(|x| **x <= frag || *x%frag != 0).copied().collect();
        frag += 1;
    }

    for res in result {
        print!("{} ", res);
    }
}
