/*
 * Lifetime Annotations:
 * Help the compiler make sure refs won't outlive the value they refer to.
 *
 * Rule 10: References to a value can't outlive the value they refer to.
 */

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
    let result;

    {
        let languages = vec![
            String::from("rust"),
            String::from("go"),
            String::from("typescript"),
        ];

        result = next_language(&languages, "go");

        // 'languages' goes out of scope, value is dropped!
    }

    println!("{}", result);
}
