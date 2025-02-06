use proconio::input;

fn main() {
    input! {
        n: i64,
        s: i64,
        a: [i64; n]
    }

    for b in 0..(1_i64 << n) {
        let sum_list = (0..n)
            .filter(|x| ((b & (1 << x)) != 0))
            .map(|x| a[x as usize]);
        let mut sum_value = 0;
        sum_list.for_each(|x| {
            sum_value += x;
        });
        if s == sum_value {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
