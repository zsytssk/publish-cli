pub fn copy_to_clipboard(s: &str) {
    cli_clipboard::set_contents(s.to_owned()).unwrap();
}
