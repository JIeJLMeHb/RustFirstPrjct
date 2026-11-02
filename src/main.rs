macro_rules! custom_print {
    ($msg:expr) => {
        println!("========== {} ==========", $msg);
    };
}

fn main() {
    custom_print!("Hello world!");
}
