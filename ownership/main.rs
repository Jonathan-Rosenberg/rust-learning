fn main() {
    let s1: String = String::from("what?!");

    let len = calculate_len(&s1); // passing reference

    println!("Length of '{}' is {}.", s1, len);
}

fn calculate_len(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.