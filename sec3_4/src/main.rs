use proconio::input;

fn main() {
    input!{
        n: i32,
    }

    let mut ans = 1.0;

    for i in 1..n {
        // let r = (n-i) as f64 / n as f64;
        ans += n as f64 / i as f64;
        // println!("{}",ans);
    }

    println!("{}", ans);
}