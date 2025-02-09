use proconio::input;

fn euclid(mut a: i64, mut b: i64) -> i64 {
    if a < b {
        let c = a;
        a = b;
        b = c;
    }
    let over = a % b;
    if over == 0 {
        b
    } else {
        euclid(b, over)
    }
}

fn main() {
    input! {
        n: i32,
        a: [i64;n]
    }

    let mut factor = euclid(a[0], a[1]);
    let mut multiple = factor * (a[0]/factor) * (a[1]/factor);

    for i in 2..n {
        factor = euclid(multiple, a[i as usize]);
        multiple = factor * (multiple/factor) * (a[i as usize]/factor);
    }

    println!("{}", multiple);

}
