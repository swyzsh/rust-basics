fn get_first_word(sentence: &str) -> &str {
  for (index, char) in sentence.chars().enumerate() {
    if char == ' ' {
      return &sentence[0..index];
    }
  }
  sentence
}

fn get_last_word(sentence: &str) -> &str {
  if let Some(index) = sentence.rfind(' ') {
    return &sentence[index + 1..];
  }
  sentence
}

fn main() {
  // if use &str instead of String,
  // remove & borrow from &sentence
  // let sentence: &str = "Hello world! My name is Swyzsh!";
  let sentence: String = String::from("Hello world! My name is Swyzsh!");
  let first_word = get_first_word(&sentence);
  let last_word = get_last_word(&sentence);

  println!("Full Sentence is: {}", sentence);
  println!("First word is: '{}' | Last word is: '{}'", first_word, last_word);
}
