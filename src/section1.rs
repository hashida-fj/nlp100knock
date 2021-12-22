use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Display;

// q0
pub fn reverse(text: &str) -> String {
    text.chars().rev().collect()
}

// q01
pub fn odd_char(text: &str) -> String {
    text.chars().step_by(2).collect()
}

// q02
pub fn concat_alternately(left: &str, right: &str) -> String {
    let mut result = String::new();
    left.chars().zip(right.chars()).for_each(|(l, r)| {
        result.push(l);
        result.push(r);
    });
    result
}

// q03
pub fn counts_word_length(text: &str) -> Vec<usize> {
    let punctations = &['(', ')', ',', '\"', '.', ';', ':', '\''][..];
    text.split_whitespace()
        .map(|word| word.replace(punctations, "").len())
        .collect()
}

// q04
pub fn q04() -> HashMap<String, usize> {
    let org_str = "Hi He Lied Because Boron Could Not Oxidize Fluorine. New Nations Might Also Sign Peace Security Clause. Arthur King Can.";
    let take1st = [1, 5, 6, 7, 8, 9, 15, 16, 19];
    let punctations = &['(', ')', ',', '\"', '.', ';', ':', '\''][..];

    org_str
        .replace(punctations, "")
        .split_whitespace()
        .enumerate()
        .map(|(i, s)| {
            let k = if take1st.contains(&(i + 1)) { 1 } else { 2 };
            (s.get(0..k).unwrap().to_string(), i + 1)
        })
        .collect::<HashMap<_, _>>()
}

pub fn character_n_gram(text: &str, n: usize) -> Vec<String> {
    (0..text.len() - n + 1)
        .map(|i| text.get(i..i + n).unwrap().to_string())
        .collect()
}

pub fn word_n_gram(text: &str, n: usize) -> Vec<Vec<String>> {
    let words: Vec<String> = text.split_whitespace().map(|w| w.to_string()).collect();

    (0..words.len() - n + 1)
        .map(|i| {
            println!("{}", i);
            words.get(i..i + n).unwrap().to_vec()
        })
        .collect()
}

pub fn set(left: &str, right: &str) {
    let left_set: HashSet<String> = character_n_gram(left, 2).into_iter().collect();
    let right_set: HashSet<String> = character_n_gram(right, 2).into_iter().collect();

    println!(" left set: {:?}", left_set);
    println!("right set: {:?}", right_set);
    let union = &left_set | &right_set;
    let intersection = &left_set & &right_set;
    let diff = &left_set - &right_set;

    println!(" union: {:?}", union);
    println!(" intersection set: {:?}", intersection);
    println!(" diff: {:?}", diff);

    println!("{}", left_set.contains(&"se".to_string()));
    println!("{}", right_set.contains(&"se".to_string()));
}

pub fn generate_template_text<X: Display, Y: Display, Z: Display>(x: X, y: Y, z: Z) -> String {
    format!("{}時の{}は{}", x, y, z)
}

pub fn typoglycemia(text: &str) -> String {
    let mut rng = thread_rng();

    let ret: Vec<_> = text
        .split_whitespace()
        .map(|word| {
            let mut chars: Vec<char> = word.chars().collect();
            let l = chars.len();
            match l {
                0 | 1 | 2 | 3 | 4 => word.to_string(),
                _ => {
                    /*
                    println!("chars -> {:?}", chars);
                    println!("word len {} in {:?}", word.len(), word);
                    println!("chars len {}", chars.len());
                    */
                    let inner_slice = chars.as_mut_slice();
                    inner_slice[1..l - 1].shuffle(&mut rng);
                    chars.into_iter().collect()
                }
            }
        })
        .collect();

    println!("{}", ret.join(" "));
    ret.join(" ")
}

pub fn run_section1() {
    println!("{}", reverse("stressed"));
    println!("{}", reverse("ぱたとくかしーー"));

    println!("{}", odd_char("stressed"));
    println!("{}", odd_char("ぱたとくかしーー"));

    println!("{}", concat_alternately("張とかー", "タクシー"));

    println!("{:?}", counts_word_length("Now I need a drink, alcoholic of course, after the heavy lectures involving quantum mechanics."));

    println!("{:?}", q04());

    println!("{:?}", character_n_gram("I am an NLPer", 4));
    println!("{:?}", word_n_gram("I am an NLPer", 2));

    set("paraparaparadise", "paragraph");

    println!("{}", generate_template_text(12, "気温", 22.4));

    typoglycemia("I couldn’t believe that I could actually understand what I was reading : the phenomenal power of the human mind");
    typoglycemia("こんにちは みなさん おげんき ですか？ わたしは げんき です");
}
