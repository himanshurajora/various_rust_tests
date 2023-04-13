use std::collections::HashMap;

fn main() {
    let input = String::from("aaaabbbbcccddee");

    let mut map: HashMap<&char, u16> = HashMap::new();

    for i in input.chars() {
        let index = i;
        let index2 = i;
        if let Some(value) = map.get(&index) {
            map.insert(&index2, *value + 1);
            continue;
        }
    }
}
