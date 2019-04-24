pub fn flshift(words: &[String], shift_amount: usize) -> String {
    let mut out = words
        .iter()
        .zip(
            words
                .iter()
                .map(|word| word.chars().nth(0).unwrap())
                .cycle()
                .skip(shift_amount),
        )
        .map(|(word, letter)| {
            let mut word: String = word.chars().skip(1).collect();
            word.insert(0, letter);
            word
        })
        .fold(String::new(), |mut s, word| {
            s.push_str(&word);
            s.push(' ');
            s
        });

    out.pop();

    out
}
