use std::collections::HashMap;

fn main() {
//    let v: Vec<i32> = Vec::new();
    let mut v = vec![3, 2, 1];

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let third = v[2];
    let tenth = v.get(10);

    println!("Third: {}", third);

    match tenth {
        None => println!("No tenth value"),
        Some(n) => println!("Tenth value: {}", n),
    }

//    v.push(23);

    println!("Vec: {:?}", v);

    let v1 = vec![23, 34, 45];

    // without the &v1, line 31 will fail with a move error
    for i in &v1 {
        println!("Val: {}", i);
    }

    println!("Vec1: {:?}", v1);

    let mut v = vec![3, 2, 1];

    for i in &mut v {
        *i += 50;
    }

    println!("Vec: {:?}", v);

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    };

    let row = vec![
        SpreadsheetCell::Int(23),
        SpreadsheetCell::Float(3.14),
        SpreadsheetCell::Text(String::from("foo"))
    ];

    for v in &row {
        match v {
            &SpreadsheetCell::Int(ref i) => println!("Row: {:?}", i),
            &SpreadsheetCell::Float(ref f) => println!("Row: {:?}", f),
            &SpreadsheetCell::Text(ref t) => println!("Row: {:?}", t),
        }
    }

    strings();

    hashmaps();
}

fn strings() {
    let data = "this is a test";
    let s = data.to_string();
    let s1 = "initial contents".to_string();

    // after this next statement, `s` will have been moved, and won't be usable any more
    // So although let s3 = s1 + &s2; looks like it will copy both strings and create a
    // new one, this statement actually takes ownership of s1, appends a copy of the
    // contents of s2, and then returns ownership of the result. In other words, it
    // looks like it’s making a lot of copies but isn’t: the implementation is more
    // efficient than copying.
    let s2 = s + &s1;

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);

    println!("s: {}", s);

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
}

fn hashmaps() {
    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("red"), 50);

    println!("Map: {:?}", scores);

    let teams = vec![String::from("blue"), String::from("yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    println!("Scores: {:?}", scores);

    // For types that implement the Copy trait, like i32, the values are copied
    // into the hash map. For owned values like String, the values will be moved
    // and the hash map will be the owner of those values
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    println!("{} = {:?}", &team_name, score);
    println!();

    for (k, v) in &scores {
        println!("{} = {}", k, v);
    }

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // replace existing value, or insert new
    scores.insert(String::from("Blue"), 23);
    println!("Scores: {:?}", scores);

    // These calls will only insert a value if that key isn't already present
    scores.entry(String::from("Blue")).or_insert(99);
    scores.entry(String::from("Red")).or_insert(99);
    println!("Scores: {:?}", scores);

    word_count();
}

fn word_count() {
    let text = "hello world wonderful world".to_string();

    let mut words = HashMap::new();

    for word in text.split_whitespace() {
        let count = words.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", words);
}