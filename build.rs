use structopt::clap::Shell;

include!("src/cli.rs");

fn main() {
    let outdir = std::env::var("OUT_DIR").unwrap();
    let mut app = BaseCommand::clap();
    app.gen_completions("bodhi-cli", Shell::Bash, outdir);
}
