use std::collections::HashSet;

/// This is the tokeniser and the pre-processor
/// 
/// This function does a few things:
/// - It tokenises the code by splitting every whitespace character
/// - It appends every line (Which is now a vector of tokens) to another vector.
///     - The index of the line in this vector is also its line number - 1.
/// - It removes all comments
/// 
/// We get an end result like this:
/// ```
///  //line 1       //line 2
/// [["Hello"], ["Hi", "there"]]
/// ```
pub fn tokenize(lines: Vec<String>) -> Vec<Token> {

    let mut tokenised_lines: Vec<Vec<String>> = Vec::new();


    // SEPARATE TOKENS BY WHITESPACE AND SPECIAL CHARACTERS

    // A set of special characters to separate
    let special_chars: HashSet<char> = [ // for clarity:
        '(', ')',                       // brackets
        '{', '}',                       // curly brackets
        '[', ']',                       // square brackets
        '<', '>',                       // smaller and greater signs
        '\'', '"',                      // apostrophe and quotation mark
        '!', '|', '&',                  // exclamation mark, or operator, and operator
        ',', '.', ':', ';',             // comma, period, colon, semicolon
        '+', '*', '/', '-', '=', '^',   // mathematical operators: plus, multiplication, 
                                        // division, minus, equals, power
        ].iter().cloned().collect();

    for line in lines {
        // Vector to hold the tokens of the current line
        let mut tokens = Vec::new();
        // String to hold the current token
        let mut token = String::new();

        // Iterate through every character
        for ch in line.chars() {
            // If that character is a space, add the token variable
            // the tokens vector and clear the token variable
            if ch.is_whitespace() {
                if !token.is_empty() { // Sometimes there was nothing here before
                    tokens.push(token);
                }
                token = String::new();
            }
            // If the character is a special token, add the token variable
            // to the tokens vector, as well as the special character as
            // another token.
            else if special_chars.contains(&ch) {
                if !token.is_empty() {
                    tokens.push(token);
                }
                token = String::new();
                tokens.push(ch.to_string())
            }
            // Otherwise, it is just a normal character part of a normal word,
            // so just push it to the token variable.
            else {
                token.push(ch);
            }
        }

        // If the last token is not empty, push it to the tokens vector
        if !token.is_empty() {
            tokens.push(token);
        }

        // Push the tokens vector to the tokenised_lines vector
        // This is equal to adding one line to the vector
        tokenised_lines.push(tokens);
    }

    println!("tokens: {:?}",tokenised_lines);

    // REMOVE COMMENTS

    // Run through the lines.
    // If two "/" tokens are found consecutively, delete them as well as
    // the rest of the line.
    for line_index in 0..tokenised_lines.len() {
        let line = &mut tokenised_lines[line_index];

        if line.len() > 1 { // ignore lines shorter than 2 characters
            for i in 0..(line.len() - 1) { // -1 : no need to check last character
                if line[i] == "/" && line[i+1] == "/" { // if two consecutive "/"s are found
                    line.truncate(i);   // cut off the rest of the line
                    break;  // exit the loop because otherwise we's be iterating over nothing.
                }
            }
        }
    }


    println!();
    println!("commentless tokens: {:?}", tokenised_lines);



    // Turn everything into a Token struct.
    // This struct contains the token itself as a String
    // and other information such as what line it's in and
    // its position in that line.
    // Originally indices in the Vec<Vec<String>> were used
    // as line count and token position, but it turns out
    // it's easier to have a continuous stream of tokens.
    let mut tokens: Vec<Token> = Vec::new();

    for (line_number, line) in tokenised_lines.iter().enumerate() {
        for (token_number, token) in line.iter().enumerate() {
            tokens.push(
                Token {
                    token        : token.clone(),
                    line         : line_number,
                    token_number : token_number,
                }
            )
        }
    }
    println!("");
    println!("Token struct: {:?}", tokens);

    return tokens;

}

#[derive(Debug, Clone)]
pub struct Token {
    pub token: String,          // the token itself
    pub line: usize,            // which line this at
    pub token_number: usize,    // which token in the line this is (1st, 2nd...)
}
