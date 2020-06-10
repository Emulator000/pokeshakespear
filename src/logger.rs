use chrono::Local;

pub fn log<S: AsRef<str>>(text: S) {
    println!(
        "{} - {}",
        Local::now().format("%Y-%m-%d %H:%M:%S"),
        text.as_ref()
    );
}
