/*
 * Lifetime Annotations:
 * Help the compiler make sure refs won't outlive the value they refer to.
 *
 * Rule 10: References to a value can't outlive the value they refer to.
 */

/*
 * last_language(..):
 *
 * Returns the last element in the vector.
 *
 * Rust assumes the returned ref is tied to the only arg.
 * You could add in lifetime annotations, but they are not required.
 * We have to think about annotations anytime your function receives a ref and
 * returns a ref.
 * Omitting lifetime annotations is referred to as *elision*, from the English
 * verb "to elide".
 * Examples:
 *   - "I *removed* the lifetime annotations" -> "I *elided" the lifeline
 * annotations";
 *   - "We can *remove* the annotations" -> "We can *elide* the annotations";
 *   - "Think about *removal* of the annotations" -> "Think about *elision* of
 *   the annotations".
 */
fn last_language(languages: &[String]) -> &str {
//fn last_language<'a>(languages: &'a [String]) -> &'a str {
    languages.last().unwrap()
}

/*
 * next_language(..):
 *
 * Finds a given language and returns the next one.
 *
 * Function that takes in two refs and returns a ref.
 *
 * If you have a function that takes in two or more refs, and returns a ref,
 * Rust will make a huge assumption: Rust assumes that the return ref will point
 * at data referred to by one of the arguments.
 * Rust will not analyze the body of your function to figure out whether the
 * return ref is pointing at the first or second arg.
 * To clarify which ref the return ref is pointing at, we have to add lifetime
 * annotations. You can think of it like an identifier.
 * 'a could be anything like 'LifetimeAnnotation, but by convention, we name it
 * 'a.
 */
fn next_language<'a>(languages: &'a [String], current: &str) -> &'a str {
    let mut found = false;

    for language in languages {
        if found {
            // Returns a ref to a language.
            return language;
        }

        if language == current {
            found = true;
        }
    }

    // Returns a ref to a language.
    languages.last().unwrap()
}

/*****************
 * Main Function *
 *****************/
fn main() {
    let languages = vec![
        String::from("rust"),
        String::from("go"),
        String::from("typescript"),
    ];

    //let result = next_language(&languages, "go");

    let result = last_language(&languages);

    println!("{}", result);
}
