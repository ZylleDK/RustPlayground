fn main() {

    // Immutable string, cannot be changed! (Stored on the stack)
    let title = "Fun with strings";

    // Funktionen 'println!()' fungerer som fstrings fra Python. 
    println!("{title}!");

    
    




    // Mutable string can be changed after the fact! (Stored on the heap)
    let mut goodbye = String::from(title);
    goodbye.push_str(" is now over! Goodbye!");

    println!("{goodbye}");
}