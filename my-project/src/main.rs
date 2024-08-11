use std::collections::HashMap;

fn main() {
    let mut hm = HashMap::new();

    hm.insert("foo", Vec::new());

    hm.get_mut("foo").unwrap().push(32);

    let bar = hm.get("foo").unwrap().get(0).unwrap();
    println!("{bar}");
}
