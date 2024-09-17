use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 20);
    scores.insert(String::from("Red"), 60);

    let mut my_team = String::from("Red");
    
    let my_score = scores.get(&my_team).unwrap_or(&0);
    println!("borrowed score: {my_score}");

    let my_score = scores.get(&my_team).copied().unwrap_or(0);
    println!("coppied score: {my_score}");

    let my_team_copy = &mut my_team;
    my_team_copy.push('s');
    let my_team_copy2 = &my_team;
    println!("team change: {my_team_copy2}");
    
    let my_team_copy2 = my_team_copy2.to_string();
    scores.insert(my_team_copy2, 80);

    let score = scores.get(&String::from("Reds")).copied().unwrap_or(0);
    println!("new score: {score}");

    println!("hashmap: {scores:?}");

    let text = "some good some bad some wonderful";

    let mut words = HashMap::new();

    for word in text.split_whitespace() {
        let count = words.entry(word).or_insert(0);
        *count += 1;
    }

    println!("words: {words:?}");
}
