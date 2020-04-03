fn main() {
    let num = vec![34, 23, 67, 88];
    let large = farge(&num);

    println!("Largest is {}", large);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let large = farge(&char_list);

    println!("Largest is {}", large);

    let int = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    println!("{:?} {:?}", int, float);
    let both_integer = SPoint { x: 5, y: 10 };
    let both_float = SPoint { x: 1.0, y: 4.0 };
    let integer_and_float = SPoint { x: 5, y: 4.0 };
    println!(
        "{:?} {:?} {:?}",
        both_integer, both_float, integer_and_float
    );
}

// fn farge<T: PartialOrd + Copy>(list: &[T]) -> T {

fn farge<T>(list: &[T]) -> T
where
    T: PartialOrd + Copy,
{
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}
#[derive(Debug)]

struct SPoint<T, U> {
    x: T,
    y: U,
}

// impl Debug for Point {}

// fn farge<T>(list: &[T]) -> &T
// where
//     T: PartialOrd,
// {
//     let mut largest = &list[0];

//     for item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }
//     &largest
// }

// fn largest_i32(n: &[i32]) -> i32 {
//     let mut largest = n[0];
//     for &item in n.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }
// fn largest_char(n: &[char]) -> char {
//     let mut largest = n[0];
//     for &item in n.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }
