pub fn run() {
    let sentence: String = String::from("Hi Abhishek");
    let first_word: String = get_first_word(sentence);
    println!("{}", first_word);
}

fn get_first_word(sentence: String) -> String {
    let mut ans: String = String::from("");
    for char in sentence.chars() {
        ans.push_str(char.to_string().as_str());
        if char == ' ' {
            break;
        }
    }
    return ans;
}
