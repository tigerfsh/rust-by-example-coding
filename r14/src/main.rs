struct A;

struct Single(A);

struct SingleGen<T>(T);

fn main() {
    // Generics
    let _s = Single(A);

    let _char = SingleGen('a');
    let _t = SingleGen(A);
    let _i32 = SingleGen(6);
    let _char = SingleGen('B');
}
