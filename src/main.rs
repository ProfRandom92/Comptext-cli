mod cli;
mod cli_p1;
mod contracts;
mod provider;

fn main() {
    let code = cli::run(std::env::args().skip(1));
    std::process::exit(code);
}
