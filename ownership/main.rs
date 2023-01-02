fn main() {
    let mut s1: String = String::from("what?!");

    let len = calculate_len(&s1); // passing reference

    println!("Length of '{}' is {}.", s1, len);

    mutating(&mut s1);
    println!("Mutated string: '{}'", s1);
}

fn calculate_len(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.

fn mutating(v: &mut String) {
    v.push_str("- added this string");
}