use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a1:i32,
        a2:i32,
        a3:i32,
    }

    let res: i32 = a1 * a2 * a3;
    println!("{}", res)
}
