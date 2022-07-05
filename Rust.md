# Rust

```rust
let a: Vec<i32> = vec![];
let mut b = a;
b.push(5);

println!("a size: {}", a.len());
println!("b size: {}", b.len());
```

**The three types**

- Value
- Reference (you can read data from that value)
- Mutable Reference (you can also mutate)

The three simple rules

**Rule #1: Value**

Only one value owner.

### Note

If the object implements copy, it can be implicitly copied

```rust
let x = 5;
let y = x;
```

```rust
println!("{}", x + y);
```

**Rule #2: Reference**

You can have as many references as you like with the constraint that there are no mutable references alive.

```rust
let x = 5;
let y = &x;
```

```rust
println!("here is {} and {}", x, y);
```

**Rule #3:1 Mut Reference**

You can have one mut reference and no reference at the same time.

```rust
fn main() {
let mut x = 5;
let y = &x;
let z = &mut x; // cannot borrow x as mutable
// because its already as immutable
println!("{}", x + y + z);
}
```

## Enum

```rust
enum Option2 {
    Baz,
    Foo(isize),
    Bar(String),
    Fuzz(Vec<String>), // string[], or a []string
}

fn main() {
    let opt2 = Option2::Foo(5);

    let mut opt22 = Option2::Fuzz(vec![]);

    if let Option2::Foo(x) = opt2 {
        let _ = x + 5;
        // x = 7;
    }

    if let Option2::Fuzz(vec) = &mut opt22 {
        vec.push(String::from("Hello, world!"));
    }

    match opt2 {
        Option2::Baz => todo!(),
        Option2::Foo(_) => todo!(),
        Option2::Bar(_) => todo!(),
        Option2::Fuzz(_) => todo!(),
    }
}
```

## generic types

**But how is this practically useful?**

3 things.

- lists with many types
- Nullable
- Error Handling

```rust
struct Foo {
    bar: Option<i32>
}

fn main() {
    let foo = Foo {
        bar: None
    };

    let foo2 = Foo {
        bar: Some(2)
    };

    if foo.bar.is_some() {
        let sum = foo.bar.unwrap() + 5;
    }

    foo.bar.unwrap_or(0);

    foo.bar.unwrap_or_else(|| {
        return 5;
    });

    let out = foo.bar.map(|x| {
        return x + 5;
    });
}
```

### Lets implement the Option enum!

```rust
enum Option2<T> {
    None,
    Some(T)
}

impl<T> Option2<T> {
    pub fn map(&self, f: fn(&T) -> T) -> Option2<T> {
        return match self {
            Option2::None => Option2::None,
            Option2::Some(v) => Option2::Some(f(v)),
        }
    }

    pub fn is_some(&self) -> bool {
        return match self {
            Option2::None => false,
            Option2::Some(_) => true,
        }
    }
}

fn main() {
    let opt = Some(5);
    let opt2 = Option2::Some(5);

    opt.map(|x| x + 5);
    let opt2 = opt2.map(|x| x + 5);

    if opt2.is_some() {

    }
}
```
