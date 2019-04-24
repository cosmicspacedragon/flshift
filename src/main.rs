use structopt::StructOpt;

fn main() {
    let opts = Opts::from_args();

    println!("{}", flshift(&opts.words, opts.shift_amount as usize));
}

fn flshift(words: &[String], shift_amount: usize) -> String {
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

#[derive(StructOpt, Debug)]
#[structopt(
    about = "A tool to shift the first letter of every word in a sequence of words `n` times to the right"
)]
#[structopt(rename_all = "kebab-case")]
struct Opts {
    words: Vec<String>,
    #[structopt(short = "n", default_value = "1")]
    shift_amount: u8,
}
