enum Option<T> {
    None,
    Some(T),
}

impl<T> Option<T> {
    fn is_some(&self) -> bool {
        return match self {
            Option::None => false,
            Option::Some(_) => true,
        };
    }
}

fn main() {
    let foo = Option::Some(5);

    if foo.is_some() {
        let value = foo.unwrap();
    }
}
