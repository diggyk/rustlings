// iterators2.rs
// In this module, you'll learn some of unique advantages that iterators can offer.
// Step 1. Complete the `capitalize_first` function to pass the first two cases.
// Step 2. Apply the `capitalize_first` function to a vector of strings.
//         Ensure that it returns a vector of strings as well.
// Step 3. Apply the `capitalize_first` function again to a list.
//         Try to ensure it returns a single string.
// As always, there are hints if you execute `rustlings hint iterators2`!

pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().to_string() + c.as_str(),
    }
}

pub fn capitalize_words(strings: Vec<&str>) -> Vec<String> {
    let mut new_strings: Vec<String> = Vec::new();
    for string in strings {
        new_strings.push(capitalize_first(string).clone());
    }

    new_strings
}

pub fn capitalize_words_to_str(strings: Vec<&str>) -> String {
    let mut capitalized_words: Vec<String> = capitalize_words(strings);

    if capitalized_words.len() == 1 {
        let lone_word = capitalized_words.pop().unwrap();
        return String::from(lone_word)
    }

    let mut new_string = String::from(capitalized_words.first().unwrap());

    for string in &capitalized_words[1..] {
        new_string = format!("{}{}", new_string, string);
    }

    new_string.to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Step 1.
    // Tests that verify your `capitalize_first` function implementation
    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    // Step 2.
    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        let capitalized_words = capitalize_words(words);
        assert_eq!(capitalized_words, ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        let capitalized_words = capitalize_words_to_str(words);
        assert_eq!(capitalized_words, "Hello World");
    }
}
