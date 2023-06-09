fn main() {

    // Immutable string, cannot be changed! (Stored on the stack)
    let title = "Fun with strings";

    // Funktionen 'println!()' fungerer som fstrings fra Python. 
    println!("{title}!");
    
    let mut mutable_string = String::from(title);
    mutable_string.push_str(" is fun for everyone!");
    println!("{mutable_string}");

    let mut i: u16 = 0;

    while i < 0xFFFF {
        mutable_string = String::from("test");
        println!("{mutable_string}{i}");
        i = i + 1;
    }




    // Mutable string can be changed after the fact! (Stored on the heap)
    let mut goodbye = String::from(title);
    goodbye.push_str(" is now over! Goodbye!");

    println!("{goodbye}");
}