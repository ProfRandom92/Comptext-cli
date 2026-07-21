mod cli;
mod provider;
mod contracts;
mod cli_p1;

fn main() {
    let code = cli::run(std::env::args().skip(1));
    std::process::exit(code);
}
