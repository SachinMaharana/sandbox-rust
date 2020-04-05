fn main() {
    println!("{}, {}", quartic_root(100f32), quartic_root_u(100f64));

    println!("{}, {}", g_quartic(-100f32), g_quartic(-100f64));
    let mut a = 'A';
    let mut b = 'B';
    println!("{}, {}", a, b);

    std::mem::swap(&mut a, &mut b);
    println!("{}, {}", a, b);
}

fn quartic_root(num: f32) -> f32 {
    return num.sqrt().sqrt();
}

fn quartic_root_u(num: f64) -> f64 {
    return num.sqrt().sqrt();
}

fn g_quartic<Number>(x: Number) -> Number
where
    Number: HasSquareRootAbs,
{
    x.abs().sqrt().sqrt()
}

trait HasSquareRootAbs {
    fn sqrt(self) -> Self;
    fn abs(self) -> Self;
}

impl HasSquareRootAbs for f32 {
    fn sqrt(self) -> Self {
        f32::sqrt(self)
    }
    fn abs(self) -> Self {
        f32::abs(self)
    }
}

impl HasSquareRootAbs for f64 {
    fn sqrt(self) -> Self {
        f64::sqrt(self)
    }
    fn abs(self) -> Self {
        f64::abs(self)
    }
}

fn _f1<T>(a: T) -> T {
    a
}
fn _f2<T>(a: T) -> T {
    let b: T = a;
    let mut c = b;
    c = _f1(c);
    c
}

fn _f3<T>(a: &T) -> &T {
    a
}
