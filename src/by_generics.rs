fn main() {
    let _s = Single(A);
    let _char: SingleGen<char> = SingleGen('a');

    let _t = SingleGen(A);
    let _i32 = SingleGen(6);
    let _char = SingleGen('a');
}

// fn foo<T>(args: T) {}

struct A;
struct Single(A);

struct SingleGen<T>(T);

// struct S(A);
// struct SGen<T>(T);

// fn reg_fn(_s: S) {}

// fn gen_spec_t(_s: SGen<A>) {}
