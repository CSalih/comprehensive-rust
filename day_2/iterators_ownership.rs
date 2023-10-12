fn iterator() {
    let v: Vec<i8> = vec![10, 20, 30];
    let mut iter = v.iter();

    println!("v[0]: {:?}", iter.next());
    println!("v[1]: {:?}", iter.next());
    println!("v[2]: {:?}", iter.next());
    println!("No more items: {:?}", iter.next());
}

fn iterator_type() {
    let v: Vec<i8> = vec![10, 20, 30];
    let mut iter = v.iter();

    let v0: Option<&i8> = iter.next();
    println!("v0: {v0:?}");
}

fn into_iterator() {
    let v: Vec<String> = vec![String::from("foo"), String::from("bar")];
    let mut iter = v.into_iter();

    let v0: Option<String> = iter.next();
    println!("v0: {v0:?}");
}

fn for_loops() {
    let v: Vec<String> = vec![String::from("foo"), String::from("bar")];

    // type of word is &String
    for word in &v {
        println!("word: {word}");
    }

    // type of word is String
    for word in v {
        println!("word: {word}");
    }
}

fn main() {
    iterator();
    iterator_type();
    into_iterator();
    for_loops();
}
