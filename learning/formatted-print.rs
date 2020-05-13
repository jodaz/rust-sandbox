fn main() {
    println!("{} days ", 31);

    // Alice this is Bob, Bob this is Alice
    println!("{0}, this is {1}, {1} this is {0}", "Alice", "Bob");

    println!("{subject} {verb} {object}",
            object="the lazy dog",
            subject="The quick brown fox",
            verb="jumps over"
            );
}
