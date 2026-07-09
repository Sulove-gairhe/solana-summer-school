#[derive(Debug)]
struct Wrapper<T> {
    label: String,
    content: T,
}

fn print_wrapper<T: std::fmt::Debug>(wrapper: Wrapper<T>) {
    println!("{:?}", wrapper);
}

fn main() {
    let number_wrapper = Wrapper {
        label: String::from("My Balance"),
        content: 1000,
    };

    let name_wrapper = Wrapper {
        label: String::from("My Name"),
        content: "Gairhe ji",
    };

    print_wrapper(number_wrapper);
    print_wrapper(name_wrapper);
}
