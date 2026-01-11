use clap::{App, Arg};

fn main() {
    let matchs = App::new("echor")
        .author("Kashif Yousuf, <kashifyousuf.sc@gmail.com")
        .about("Rust Echo CLI")
        .version("0.1.0")
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .help("Input Text")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("omit_newline")
                .short("n")
                .help("Dont print newline")
                .takes_value(false),
        )
        .get_matches();

    let text = matchs
        .values_of_lossy("text")
        .expect("Unable to Parse/find the arg: TEXT");
    let omit_newline = matchs.is_present("omit_newline");

    print!("{}{}", text.join(" "), if omit_newline { "" } else { "\n" });
}
