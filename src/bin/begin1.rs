fn main() {
    let mut arr = [1, 2, 6, 4, 7, 8];
    use std::cmp::Ordering;

    arr.sort_by(|a, b|
        if a < b { Ordering::Greater }
        else if a > b { Ordering::Less }
        else { Ordering::Equal });
    print!("{:?}\n", arr);

    let y = arr.clone( );

    y.sort_by(|a,b| a.cmp(b));
    print!("{:?}\n", y);

}


// fn desc(a: &i32, b: &i32) -> Ordering {
//     if a < b {
//         Ordering::Greater
//     } else if a > b {
//         Ordering::Less
//     } else {
//         Ordering::Equal
//     }
// }