pub fn create_printer(verbose: bool) -> impl Fn(&str) {
    move |msg| {
        if verbose {
            println!("{}", msg);
        }
    }
}
