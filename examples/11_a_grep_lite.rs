use regex::Regex;

fn main() {
    let re = Regex::new("picture").unwrap(); // non-idiomatic way of handling errors

    let quote = "\
Every face, every shop,
bedroom window, public house, and
dark square is a picture
feverishly turned--in search of what?
It is the same with books.
What do we seek
Through millions of pages?";

    for line in quote.lines() {
        let contains_substring = re.find(line);
        match contains_substring {
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}
