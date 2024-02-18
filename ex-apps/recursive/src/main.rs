fn recursive(x: usize) {
    if x == 0 {
        return;
    }
    let mut stuff = Vec::with_capacity(x);
    for i in 0..x {
        stuff.push(i);
    }
    recursive(x - 1)
}

fn main() {
    recursive(50_000);
}
