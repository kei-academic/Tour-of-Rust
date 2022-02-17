struct Foo {
    x: i32,
}

fn do_someshing() -> Foo {
    Foo { x: 42 }
}

fn main() {
    let foo = do_something();
}
