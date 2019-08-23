# Bauxite
Wrap a string in a box for more aesthetic output to a console

[![Build Status](https://travis-ci.org/Doxterpepper/Bauxite.svg?branch=master)](https://travis-ci.org/Doxterpepper/Bauxite)

# Usage
To use Bauxite add `bauxite` as a dependency in your `Cargo.toml`.
```rust
[dependencies]
bauxite = { git = "https://github.com/Doxterpepper/Bauxite" }
```

Then in your source code
```rust
extern crate bauxite;

fn main() {
    let my_message = "Only those who leisurely approach that which the masses are busy about\n\
                      can be busy about that which the masses take leisurely.\n\
                      -- Lao Tsu";
    println!("{}", bauxite::BoxBuilder::new(String::from(my_message)));
}
```

```
┌──────────────────────────────────────────────────────────────────────────┐
│                                                                          │
│  Only those who leisurely approach that which the masses are busy about  │
│  can be busy about that which the masses take leisurely.                 │
│  -- Lao Tsu                                                              │
│                                                                          │
└──────────────────────────────────────────────────────────────────────────┘
```
