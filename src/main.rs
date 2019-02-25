use std::io::Write;

fn main() {
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    loop {
        print!("calc> ");
        let _ = stdout.flush();
        let mut buf = String::new();
        let _ = stdin.read_line(&mut buf);
        let result = lsbasi::Interpreter::evaluate(buf.as_str());
        println!("{}", result);
    }
}