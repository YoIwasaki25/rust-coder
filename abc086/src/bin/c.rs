use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n :usize,
        mut mat: [(i32, i32, i32); n],
    }

    mat.insert(0, (0, 0, 0));
    let mut can_go = true;

    for i in 1..n + 1 {
        let dt: i32 = mat[i].0 - mat[i - 1].0;
        let dist = (mat[i].1 - mat[i - 1].1).abs() + (mat[i].2 - mat[i - 1].2).abs();
        if dist > dt {
            can_go = false;
            break;
        }

        if (dt % 2) != (dist % 2) {
            can_go = false;
            break;
        }
    }

    println!("{}", if can_go { "Yes" } else { "No" })
}
