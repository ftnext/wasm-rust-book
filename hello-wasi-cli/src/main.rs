use ferris_says::say;

fn main() {
    let out = "Hello, world!";
    let width = 80;
    let mut writer = std::io::stdout();

    if let Err(e) = say(out, width, &mut writer) {
        println!("{e}");
    }
}
