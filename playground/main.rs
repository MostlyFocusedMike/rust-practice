fn main() {
    let mut my_name = "Pascal".to_string();
    my_name.push_str( " Precht");

    let last_name = my_name[7..];
    println!("{}", last_name);
}
