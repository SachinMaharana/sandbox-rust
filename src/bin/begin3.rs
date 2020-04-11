fn main() {
    let a = 3;
    let b = a;
    println!("{} {}", a, b);
    let c = vec![1, 2];
    let d = c.clone();
    println!("{:?} {:?}", c, d);
    let n = 12;
    let ref_n = &n;
    println!("{} {}", n, ref_n);
    let mut m = 13;
    {
        let ref_to_m = &mut m;
        *ref_to_m = 14;
    }

    // m = 16;
    // let ref2_to_m = &m;
    println!("{}", m);
    println!("*************");

    // let mut v = vec![12];
    // let ref_to_first = &v[0];
    // v.push(13);
    // println!("{}", ref_to_first);

    let _a = X('a');
    let _b;
    let _c = X('c');
    _b = X('b');

    println!("*************");
    let v1 = vec![11u8, 22];
    let result;
    {
        let v2 = vec![33u8];
        fn func<'a>(_x1: &'a Vec<u8>, _x2: &Vec<u8>) -> &'a Vec<u8> {
            _x1
        }
        result = func(&v1, &v2);
    }
    println!("{:?}", *result);
}

struct X(char);

impl Drop for X {
    fn drop(&mut self) {
        println!("Dropping : {}", self.0);
    }
}
