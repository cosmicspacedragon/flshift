use structopt::StructOpt;

fn main() {
    let opts = Opts::from_args();

    println!(
        "{}",
        flshift::flshift(&opts.words, opts.shift_amount as usize)
    );
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
