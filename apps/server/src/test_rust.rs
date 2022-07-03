// enum RSEnum {
//     Foo(i32),
//     Bar(String),
//     Baz(Vec<String>),
//     Foo2(Option<i32>),
// }

enum Option<T> {
    None,
    Some(T),
}

fn main() {
    // let a = vec![];
    // let mut b = a;

    // b.push(1);

    // println!("{:?}", b);

    // let foo = RSEnum::Foo(5);

    // // pattern matching: wow
    // if let RSEnum::Foo(value) = foo {}

    // match foo {
    //     RSEnum::Foo2(Some(value)) => {}
    //     RSEnum::Foo2(None) => {}
    //     RSEnum::Foo(value) => {}
    //     _ => {}
    // }

    let foo = Some(5);

    if let Some(value) = foo {}

    match foo {
        Some(value) => {}
        None => {}
    }

    foo.map(|x| {});

    foo.filter(|&x| x < 10);
}
