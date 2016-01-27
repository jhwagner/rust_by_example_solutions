fn main() {
    // {} will be replaced with any arguments. Arguments will be stringified
    println!("{} days", 31);

    // Positional arguments
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // Named arguments
    println!("{subject} {verb} {predicate}",
             predicate="over the lazy dog",
             subject="the quick brown fox",
             verb="jumps");

    // Special formatting specified after `:`
    println!("{} of {:b} people know binary, the other half don't", 1, 2);

    // Right-align text with specified width. Outputs "     1"
    println!("{number:>width$}", number=1, width=6);

    // Zero padding numbers. Outputs "000001"
    println!("{number:>0width$}", number=1, width=6);

    // 1st FIXME solution
    println!("My name is {0}, {1} {0}", "Bond", "James");

    // Structure which contains an `i32`
    struct Structure(i32);

    // Custom types such as this struct require more complicated handling
    // 2nd FIXME solution (comment out line as this cannot be printed by default)
    //println!("This struct `{}` won't print...", Structure(3));

    // Assignment: Print "Pi is roughly 3.143" using 22 / 7 to estimate pi
    println!("Pi is roughly {:.3}", 22.0 / 7.0);
}
