use std::collections::HashMap;

fn main() {
	let list = vec![String::from("ear"),
                    String::from("era"),
                    String::from("mug"),
                    String::from("gum"),
                    String::from("are"),
                    String::from("ear"),
                    String::from("sit")];

    let mut anagrams: HashMap<String, Vec<String>> =  HashMap::new();

    for n in list {
        let borrow_n = n.clone();
        let mut tmp = n.split("");
        let mut vec = tmp.collect::<Vec<_>>();
        let length = vec.len();
        vec.remove(length-1);
        vec.remove(0);
        vec.sort();
        let hashkey = vec.join("");
        let val = anagrams.get(&hashkey).cloned();

        match val {
            Some(x) => {
                for v in x.iter() {
                    if *v == n {
                        break;
                    }
                }
                let mut y = x.clone();
                y.push(borrow_n);
                *anagrams.get_mut(&hashkey).unwrap() = y.to_vec();
            }
            None => {
                anagrams.insert(hashkey, vec![borrow_n]);
            }
        }
    }

    // There might be elements with length 1
    // those are not anagrams, so remove them
    let anagrams_borrow = anagrams.clone();

    for (key, value) in anagrams_borrow.into_iter() {
        if value.len() == 1 {
            anagrams.remove(&key);

        }
    }

    for (_, value) in anagrams.into_iter() {
        println!("{:?}", value);
    }
}
