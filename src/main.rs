use std::io::Write;

fn main() {
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    loop {
        print!("calc> ");
        let _ = stdout.flush();
        let mut buf = String::new();
        let _ = stdin.read_line(&mut buf);
        match lsbasi::Interpreter::evaluate(buf.as_str()) {
            Ok(Some(result)) => println!("{}", result),
            Ok(None) => {},
            Err(e) => eprintln!("{}", e),
        }
    }
}
