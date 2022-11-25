
pub fn test_string() {
    let mut mut_s = String::new();
    let data = "Initial contents";
    let _s = data.to_string();
    let s = String::from("initial contents");
    println!("{}",s);
    let hello = String::from("안녕하세요");
    println!("{}", hello);

    mut_s.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    let s3 = s1 + &s2;
    println!("s1 + s2 is {}", s3 );

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);

    // --- Indexing ---
    let h = &hello[..3];
    println!("The first three bytes of {} are {}", hello, h);
    for c in hello.chars() {
        println!("{}", c);
    }
    
}