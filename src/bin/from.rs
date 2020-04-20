fn main() {
    let x: String = String::from("Hello");
    let someone = Person::new("Ok");
    println!("{:?}", someone);
}

struct SortedVec<T>(Vec<T>);

impl<'a, T: Ord + Clone> From<&'a [T]> for SortedVec<T> {
    fn from(slice: &[T]) -> Self {
        let mut vec = slice.to_owned();
        vec.sort();
        SortedVec(vec)
    }
}

impl<T: Ord + Clone> From<Vec<T>> for SortedVec<T> {
    fn from(mut vec: Vec<T>) -> Self {
        vec.sort();
        SortedVec(vec)
    }
}

#[derive(Debug)]
struct Person {
    ident: String,
}

impl Person {
    pub fn new<N>(name: N) -> Person
    where
        N: AsRef<str>,
    {
        Person {
            ident: name.as_ref().to_owned(),
        }
    }
}
