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
