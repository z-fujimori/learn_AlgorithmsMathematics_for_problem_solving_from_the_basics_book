use proconio::input;

fn main() {
    input! {
        n: i64,
        mut a: [i64;n]
    }

    let mut a_num = [0; 100001];

    // for a in a {
    //     match a {
    //         1 => a_num[0] += 1,
    //         2 => a_num[1] += 1,
    //         3 => a_num[2] += 1,
    //         // 400 => a_num[3] += 1,
    //         _ => a_num[4] += 1,
    //     }
    // }
    // println!("{:?}", a_num);
    // if a_num[4] > 0 {
    //     println!("err");
    //     panic!("エラー");
    // }

    // let result: i64 = (a_num[0] * (a_num[0]-1))/2 + (a_num[1] * (a_num[1]-1))/2 + (a_num[2] * (a_num[2]-1))/2;
    let mut result: i64 = 0;

    for a in &a {
        a_num[*a as usize] += 1;
    }

    // println!("{:?} {} {}", &a_num[..8], a_num[20000], a_num[50000]);

    for i in 1..50000 {
        result += a_num[i] * a_num[100000-i];
    }

    result += a_num[50000] * (a_num[50000]-1) / 2;

    println!("{:?}", result);
}