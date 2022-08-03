use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: i32,
        a: [i32; n],
    }

    let mut res: i32 = 0;

    for i in 0..n {
        let element: i32 = a[i as usize];
        res += element;
    }
    println!("{}", res)
}
