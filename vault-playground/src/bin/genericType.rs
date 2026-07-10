#[derive(Debug)]
struct Wrapper<T> {
    label: String,
    content: T,
}

fn print_wrapper<T: std::fmt::Debug>(wrapper: &Wrapper<T>) {
    println!("{:#?}", wrapper);
}

fn main() {
    let number_wrapper = Wrapper {
        label: String::from("number_wrapper"),
        content: 1000,
    };

    let string_wrapper = Wrapper {
        label: String::from("string_wrapper"),
        content: "Name is Gairhe ji",
    };

    print_wrapper(&number_wrapper);
    print_wrapper(&string_wrapper);
}
