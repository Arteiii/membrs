fn main() {
    #[cfg(not(unix))]
    compile_error!("This project only supports Unix-based systems.");
}
