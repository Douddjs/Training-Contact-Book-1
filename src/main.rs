use std::io::stdin;


fn main() {
    println!("Hello, world!\n\nWelcome to the contact book demo (ownership)");
    
    let mut count = String::new();
    println!("First of all, how many contacts do you want to input now? ");
    stdin().read_line(&mut count).expect("Program: Invalid Input.\n");
    let count: i32 = count.trim().parse().unwrap();
    
    let mut contacts: Vec<Contact> = Vec::new();
    
    for i in 0..count {

        println!("Contact #{}", i + 1);
        let mut phone = String::new();
        let mut name = String::new();

        println!("Name");
        stdin().read_line(&mut name).expect("Program: Invalid Input.\n");
        println!("Phone");
        stdin().read_line(&mut phone).expect("Program: Invalid Input.\n");
        println!();
        let contact = Contact {
            id: i as u32 + 1,
            name: name.trim().to_string(),
            phone: phone.trim().to_string()
        };

        
        contacts.push(contact);
    }


    println!("===CONTACTS LIST===");
    for contact in contacts {
        println!("{:?}", contact);
    }
    

}

#[derive(Debug)]
struct Contact {
    id: u32,
    name: String,
    phone: String
}
