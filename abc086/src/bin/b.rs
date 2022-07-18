use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize
    }

    let str_ab = format!("{}{}", a, b);
    let num_ab = str_ab.parse::<isize>().unwrap();

    let mut is_square = false;
    for i in 1..num_ab + 1 {
        if num_ab == i * i {
            is_square = true;
            break;
        }
    }

    if is_square {
        println!("Yes");
    } else {
        println!("No");
    }
}
