use term_tools::prelude::*;

fn main() {
    println!(
        "{}{}{}",
        "In the name of ",
        "GOD".styled().bold().cyan(),
        "."
    );

    let mut styled = "Hello World!".styled();
    styled
        .color(colors::RED)
        .format(formats::BOLD)
        .underline()
        .fg()
        .white()
        .bg();

    println!("{}", styled);

    println!("{}", "Walking dead!".styled().italic().red())
}
