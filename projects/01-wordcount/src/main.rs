use std::env;

use wordcount::{Counts, count_file};

fn main() {
    // Milestones 3–5 happen here. `env::args()` yields `String`s; the first one is the
    // program name, which is why you'll want `.skip(1)`.
    let paths: Vec<String> = env::args().skip(1).collect();

    let mut total = Counts::default();
    for path in &paths {
        // TODO(milestone 4): handle the error instead of panicking.
        let counts = count_file(path).expect("could not read file");
        print_row(&counts, path);
        total.add(counts);
    }

    // TODO(milestone 3): only print the total when there is more than one file.
    // TODO(milestone 5): with no paths, read from stdin instead.
    print_row(&total, "total");
}

fn print_row(c: &Counts, label: &str) {
    println!("{:>4} {:>4} {:>4} {label}", c.lines, c.words, c.bytes);
}
