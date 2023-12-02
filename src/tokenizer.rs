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
pub fn tokenize(lines: Vec<String>) -> Vec<Vec<String>> {

    // Tokenise the code by splitting every whitespace character.
    // Store all tokens in a vector
    // We get a Vector like this:
    // ```
    //   line 1         line 2
    // [["Hello"], ["hi", "there"]]
    // ```
    let mut tokenised_lines = Vec::new();
    for line in lines {
        // Create a new vector with every token
        tokenised_lines.push(
            line.split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>()
        );
    }
    println!("tokens: {:?}", tokenised_lines);


    // Remove all comments from the code
    // Leave blank spaces where there were comments to maintain
    // the line count
    //
    // CURRENTLY WE ONLY HAVE "//" IMPLEMENTED
    // As soon as we meet "//" -> Rest of the line becomes a comment
    // As soon as we meet "/*" -> Everything beomes a comment until "*/"
    //                                             |
    // The "comment" bool is true until "*/"  <---'
    // let mut comment: bool = true;
    for line_index in 0..tokenised_lines.len() {
        let line = &mut tokenised_lines[line_index];
    
        // Find the index of the first occurrence of "//"
        if let Some(index) = line.iter().position(|keyword| keyword == "//") {
            line.truncate(index); // Remove elements from the index to the end
        }
    }
    
    println!();
    println!("commentless tokens: {:?}", tokenised_lines);

    return tokenised_lines;

}
