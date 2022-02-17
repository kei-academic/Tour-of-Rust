struct Foo {
    x: i32,
}

fn do_something(f: &mut Foo) {
    f.x += 1;
}

fn main() {
    let mut foo = Foo { x: 42 };
    do_something(&mut foo);
    do_something(&mut foo);
}
