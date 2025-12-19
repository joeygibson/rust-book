use std::collections::HashMap;

struct Person {
    name: String,
    age: u8,
}

fn main() {
    // let v: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3, 4, 5];
    v.push(23);

    println!("{:?}", v);

    let third = &v[2];
    println!("The third element is {}", third);

    let third = v.get(23);
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let people = vec![
        Person {
            name: String::from("Alice"),
            age: 30,
        },
        Person {
            name: String::from("Bob"),
            age: 25,
        },
    ];

    for person in &people {
        println!("{} is {} years old.", person.name, person.age);
    }

    let alice = &people[0];
    println!("Alice's age is {}", alice.age);

    for i in &mut v {
        *i += 50;
    }

    println!("Mutated value = {:?}", v);

    let data = "foo bar baz";
    let s = data.to_string();
    let s2 = "hello world".to_string();
    let s3 = String::from("hello rust");
    let hello = String::from("你好");
    let output = format!("{} - {} - {} - {}", s, s2, s3, hello);
    println!("{}", output);

    let second = &hello.chars().nth(1);
    match second {
        Some(c) => println!("The second character is {}", c),
        None => println!("There is no second character."),
    }

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (team, score) in &scores {
        println!("{}: {}", team, score);
    }

    scores.entry(String::from("Green")).or_insert(100);
    println!("{:?}", scores);

    let mut map = HashMap::new();
    for word in "hello world wonderful world".split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
