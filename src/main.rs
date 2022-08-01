use kagero::printer::{Printer, Colors};

fn main() {
    let mut printer = Printer::default();
    printer.println("Hello, world!", Colors::Red);
    printer.println("This will be a very cool build system in the future!", Colors::CyanBright);
}
