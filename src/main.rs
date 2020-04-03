fn main() {
    print!("{} {}", q(100f64), q(100f32));
}

fn q<Number>(x: Number) -> Number
where
    Number: HasSquareRoot,
{
    x.sq_root().sq_root()
}
trait HasSquareRoot {
    fn sq_root(self) -> Self;
}

impl HasSquareRoot for f32 {
    fn sq_root(self) -> Self {
        f32::sqrt(self)
    }
}

impl HasSquareRoot for f64 {
    fn sq_root(self) -> Self {
        f64::sqrt(self)
    }
}
