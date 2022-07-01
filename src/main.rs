enum RSEnum {
    Foo(i32),
    Bar(String),
    Baz(Vec<String>),
    Foo2(Option<i32>),
}

fn main() {
    // let a = vec![];
    // let mut b = a;

    // b.push(1);

    // println!("{:?}", b);

    let foo = RSEnum::Foo(5);

    // pattern matching: wow
    if let RSEnum::Foo(value) = foo {}

    match foo {
        RSEnum::Foo2(Some(value)) => {}
        RSEnum::Foo2(None) => {}
        RSEnum::Foo(value) => {}
        _ => {}
    }
}
