use std::collections::HashSet;

/// This is the tokeniser and the pre-processor
/// 
/// This function does a few things:
/// - It tokenises the code by splitting every whitespace character and every
/// special character, while making sure not to split strings.
/// (everything between two `"`)
/// - It appends every line (Which is now a vector of tokens) to another vector.
///   - The index of the line in this vector is also its line number - 1.
/// - It removes all comments
/// - It returns a Vector of `Token` structs.
/// 
/// ```rust
/// pub struct Token {
///   pub token: String,        // the token itself, for example "let"
///   pub line: usize,          // which line it is at
///   pub token_number: usize,  // which token in the line this is (0st, 1st, 2nd...)
/// }
/// ```
pub fn tokenize(lines: Vec<String>) -> Vec<Token> {

  let mut tokenised_lines: Vec<Vec<String>> = Vec::new();

  // SEPARATE TOKENS BY WHITESPACE AND SPECIAL CHARACTERS.

  // A set of special characters to separate
  let special_chars: HashSet<char> = [// for clarity:
    '(', ')',                         // brackets
    '{', '}',                         // curly brackets
    '[', ']',                         // square brackets
    '<', '>',                         // smaller and greater signs
    '!', '|', '&',                    // exclamation mark, or operator, and operator
    ',', '.', ':', ';',               // comma, period, colon, semicolon
    '+', '*', '/', '-', '=', '^',     // mathematical operators: plus, multiplication, 
                                      // division, minus, equals, power
  ].iter().cloned().collect();

  for line in lines {
    // Vector to hold the tokens of the current line
    let mut tokens = Vec::new();
    // String to hold the current token
    let mut token = String::new();

    let mut is_string: bool = false;

    // Iterate through every character
    for ch in line.chars() {

      // " Marks either the end or the start of a string. If this character appears,
      // it is to be ignored and is_string variable gets inverted.
      if ch == '"' {
        is_string = !is_string;
      }
      else {
        // If we are not dealing with a string, standard separation
        // logic applies.
        if !is_string {
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
        // If we are indeed dealing with a string, push the character
        // no matter what.
        else {
          token.push(ch);
        }
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

  // REMOVE COMMENTS

  // Run through the lines.
  // If two "/" tokens are found consecutively, delete them as well as
  // the rest of the line.
  for line_index in 0..tokenised_lines.len() {
    let line = &mut tokenised_lines[line_index];

    if line.len() > 1 { // ignore lines shorter than 2 characters
      for token_index in 0..(line.len() - 1) { // -1 : no need to check last character
        if line[token_index] == "/" && line[token_index+1] == "/" { // if two consecutive "/"s are found
          line.truncate(token_index);   // cut off the rest of the line
          break;  // exit the loop because otherwise we'd be iterating over nothing.
        }
      }
    }
  }

  // Turn everything into a Token struct.
  // This struct contains the token itself as a String
  // and other information: what line it's in and
  // its position in that line.
  let mut tokens: Vec<Token> = Vec::new();

  for (line_number, line) in tokenised_lines.iter().enumerate() {
    for (token_number, token) in line.iter().enumerate() {
      tokens.push(
        Token {
          token: token.to_string(),
          line: line_number,
          token_number,
      });
    }
  }
  return tokens;
}

/// A struct used to represent each token in the code.
#[derive(Debug, Clone)]
pub struct Token {
  pub token: String,        // the token itself, for example "let"
  pub line: usize,          // which line it is at
  pub token_number: usize,  // which token in the line this is (0st, 1st, 2nd...)
}