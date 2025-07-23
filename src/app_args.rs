///Flags we can pass
#[derive(Parser, Debug)]
struct AppArgs {
    #[arg(short, long)]
    verbose: bool,
}
