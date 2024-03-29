enum Item<T> {
    Collection(Vec<Item<T>>),
    Value(T),
}

// This explore function is generic over both the type being stored (T),
// and the operation to be performed on values.
fn explore<T, F>(collection: &[Item<T>], operate: &mut F)
where F: FnMut(&T) {
    for item in collection {
        match item {
            &Item::Collection(ref items) => explore(items, operate),
            &Item::Value(ref value) => operate(value)
        }
    }
}

fn operate_i32(value: &i32) {
    println!("operate({})", value);
}

fn main() {
    use Item::*;

    let root = vec![
        Value(1),
        Collection(vec![
            Value(2),
            Value(3),
        ]),
        Value(4),
    ];

    explore(&root, &mut operate_i32)
}