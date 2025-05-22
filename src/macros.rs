#[macro_export]
macro_rules! print_with_prompt {
    ($($arg:tt)*) => {{
        let mut lock = std::io::stdout().lock();

        print!("\x1b[1K");             // delete line
        print!("\x1b[0G");             // move cursor to start of line
        println!($($arg)*);            // forward writeln!
        print!(">> ");                 // print prompt

        lock.flush().unwrap();
    }}
}
