#[macro_export]
macro_rules! print_with_prompt {
    ($($arg:tt)*) => {{
        print!("\x1b[1K");             // delete line
        print!("\x1b[0G");             // move cursor to start of line
        println!($($arg)*);            // forward println!
        print!(">> ");                 // print prompt
        std::io::stdout().flush().unwrap(); // flush prompt to stdout
    }}
}
