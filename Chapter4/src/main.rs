mod first_word;

fn main() {
    let s = String::from("Superman vs Batman");
    let i = first_word::first_word1(&s);
    println!("index: {i}");
    first_word::string_slice();
    
    
    let slice = first_word::first_word2(&s);
    println!("slice: {slice}");
}
