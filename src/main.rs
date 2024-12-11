mod container;
mod list_item;

use crate::container::{Container};

fn main() {
    let str_container = Container { value: "Thought is free." };
    println!("{}", str_container.value);
    
    let ambiguous_container: Container<Option<String>> = Container { value : None};
    let short_alt_ambiguous_container = Container::<Option<String>>::new(None);
}