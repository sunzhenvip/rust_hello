use std::collections::HashSet;

#[derive(Debug, Default)]
pub struct Difference<'a, 'b> {
    first_only: Vec<&'a str>,
    second_only: Vec<&'b str>,
}

pub fn find_difference<'a, 'b>(sentence1: &'a str, sentence2: &'b str) -> Difference<'a, 'b> {
    let sentence_1_words: HashSet<&'a str> = sentence1.split_whitespace().collect();
    let sentence_2_words: HashSet<&'b str> = sentence2.split_whitespace().collect();

    let mut diff = Difference::default();

    for word in &sentence_1_words {
        if !sentence_2_words.contains(word) {
            diff.first_only.push(word);
        }
    }

    for word in &sentence_2_words {
        if !sentence_1_words.contains(word) {
            diff.second_only.push(word);
        }
    }

    diff.first_only.sort();
    diff.second_only.sort();

    diff
}

fn main() {
    let first_sentence = String::from("I hate the surf and the sand.");
    let second_sentence = String::from("I love the surf and the sand.");

    let first_only = {
        let third_sentence = String::from("I love the snow and the sand.");
        let diff = find_difference(&first_sentence, &third_sentence);
        diff.first_only
    };

    // 注意：split_whitespace() 后 "sand." 仍然带点号
    // 你的期望里没有 "sand."，所以不会影响这两个断言
    assert_eq!(first_only, vec!["hate", "surf"]);

    let second_only = {
        let third_sentence = String::from("I love the snow and the sand.");
        let diff = find_difference(&third_sentence, &second_sentence);
        diff.second_only
    };

    assert_eq!(second_only, vec!["surf"]);
}
