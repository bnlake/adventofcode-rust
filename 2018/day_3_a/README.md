# Rust Exposures
Some of the few things I learned about rust during this puzzle

### Turning lines of a multiline string into a Vec 
```rust 
contents.lines().collect()
```

### Using more than one criteria in a `.split()` closure
```rust
.split(|x| x == ' ' || x == '@' || x == ',' || x == ':' || x == 'x' || x == '#')
```

### Using `.filter_map()` to parse a `&str` into a `usize`
```rust
.filter_map(|x| x.parse().ok())
```

### Destructure a `Vec` with a pattern slice
This must be done in an if/match block because the destructure could fail
```rust
if let [left, top, width, height] = temp[1..] {}
}
```
# Resources
https://www.reddit.com/r/adventofcode/comments/a2lesz/2018_day_3_solutions/