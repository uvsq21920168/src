use std::io;

fn main() -> io::Result<()> {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    println!("Devine mon nombre !

    Saisissez votre proposition: {}", input.trim());
    Ok(())
}
