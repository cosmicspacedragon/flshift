use structopt::StructOpt;

fn main() {
    let opts = Opts::from_args();

    opts.words
        .iter()
        .zip(
            opts.words
                .iter()
                .map(|word| word.chars().nth(0).unwrap())
                .cycle()
                .skip(opts.shift_amount as usize),
        )
        .map(|(word, letter)| format!("{}{}", letter, word.chars().skip(1).collect::<String>()))
        .for_each(|word| print!("{} ", word));

    println!();
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
