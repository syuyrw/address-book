use std::io;
use std::io::stdin;
use serde::{Serialize, Deserialize};
use std::fs::File;

// Contact struct
#[derive(Serialize, Deserialize)]
struct Contact {
    first_name: String,
    phone_number: String,
}

// New contact
fn create_contact() {
    println!("Add a new contact");

    
    // Take input for name
    println!("\nType name: ");
    let mut first_input = String::new();
    io::stdin().read_line(&mut first_input).expect("Failed to read line");
    let first_name: &str = first_input.trim();

    // Take input for phone number
    println!("\nType the phone number: ");
    let mut phone_input = String::new();
    io::stdin().read_line(&mut phone_input).expect("Failed to read line");
    let phone_number: &str = phone_input.trim();

    // Create contact
    let contact = Contact {
        first_name: first_name.to_string(),
        phone_number: phone_number.to_string(),
    };
    
    // Read existing contacts from JSON file
    let file = File::open("contacts.json").expect("Failed to open file");
    let mut contacts: Vec<Contact> = serde_json::from_reader(file).expect("Failed to read data");
    
    // Write input to contacts
    contacts.push(contact);

    // Write contacts to JSON file
    let file = File::create("contacts.json").expect("Failed to create file");
    serde_json::to_writer(file, &contacts).expect("Failed to write date");
    

    println!("Contact Added");
}

// Edit contact
fn update_contact_name(old_first_name: &str, new_first_name: &str) {
    // Read existing contacts from JSON file
    let file = File::open("contacts.json").expect("Failed to open file");
    let mut contacts: Vec<Contact> = serde_json::from_reader(file).expect("Failed to read data");
    
    // Find and update contact
    for contact in &mut contacts {
        if contact.first_name == old_first_name {
            contact.first_name = new_first_name.to_string();
            break;
        }
    }

    // Write updated contacts to JSON file
    let file = File::create("contacts.json").expect("Failed to create file");
    serde_json::to_writer(file, &contacts).expect("Failed to write data");
}

fn update_contact_phone(first_name: &str, new_phone_number: &str) {
    // Read existing contacts from JSON file
    let file = File::open("contacts.json").expect("Failed to open file");
    let mut contacts: Vec<Contact> = serde_json::from_reader(file).expect("Failed to read data");
    
    // Find and update contact
    for contact in &mut contacts {
        if contact.first_name == first_name {
            contact.phone_number = new_phone_number.to_string();
            break;
        }
    }

    // Write updated contacts to JSON file
    let file = File::create("contacts.json").expect("Failed to create file");
    serde_json::to_writer(file, &contacts).expect("Failed to write data");
}

// Edit contact
fn edit_contact() {
    println!("\nEdit contact");

    // Take user input for name of contact to edit
    println!("\nType the name of the contact you wish to edit: ");
    let mut name_input = String::new();
    io::stdin().read_line(&mut name_input).expect("Failed to read line");
    let edit_choice: &str = name_input.trim();

    // Choose whether to edit name or number
    println!("\nWhat do you want to edit? \nType '1' for contact name. \nType '2' for contact phone number");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let selection: u32 = input.trim().parse().expect("Please type 1 or 2: ");

    match selection {
        1 => {
            // Edit Name
            println!("\nType the new first name: ");
            let mut new_first_name_input = String::new();
            io::stdin().read_line(&mut new_first_name_input).expect("Failed to read line");
            let new_first_name: &str = new_first_name_input.trim();
            
            update_contact_name(edit_choice, new_first_name);

            println!("Name updated");
        },
        2 => {
            // Edit Number
            println!("\nType the new phone number: ");
            let mut new_phone_number_input = String::new();
            io::stdin().read_line(&mut new_phone_number_input).expect("Failed to read line");
            let new_phone_number: &str = new_phone_number_input.trim();
            
            update_contact_phone(edit_choice, new_phone_number);

            println!("Phone number has been updated");
        },
        _ => println!("Invalid selection!")
    }
}

// View single contact
fn view_contact() {
    println!("\nView contact");

    // Select contact to view
    println!("\nType the name of the contact you wish to view: ");
    let mut which_contact = String::new();
    io::stdin().read_line(&mut which_contact).expect("Failed to read line");
    let view_contact: &str = which_contact.trim();

    // Read existing contacts from JSON file
    let file = File::open("contacts.json").expect("Failed to open file");
    let contacts: Vec<Contact> = serde_json::from_reader(file).expect("Failed to read data");
    
    // Find and display contact
    for contact in &contacts {
        if contact.first_name == view_contact {
            println!("\nFirst name: {}", contact.first_name);
            println!("Phone number: {}", contact.phone_number);
            break;
        }
    }
    
}

fn view_all() {
    println!("\nView all contacts");

    // Read existing contacts from JSON file
    let file = File::open("contacts.json").expect("Failed to open file");
    let contacts: Vec<Contact> = serde_json::from_reader(file).expect("Failed to read data");

    // Print contacts
    for contact in &contacts {
        println!("\nName: {}", contact.first_name);
        println!("Number: {}", contact.phone_number);
        println!();
    }
}

fn delete() {
    println!("\nDelete contact");

    // Select contact to delete 
    println!("\nType the name of the contact you wish to view: ");
    let mut contact_to_delete = String::new();
    io::stdin().read_line(&mut contact_to_delete).expect("Failed to read line");
    let first_name: &str = contact_to_delete.trim();

    // Read existing contacts from JSON file
    let file = File::open("contacts.json").expect("Failed to open file");
    let mut contacts: Vec<Contact> = serde_json::from_reader(file).expect("Failed to read data");

    // Remove contact
    contacts.retain(|contact| contact.first_name != first_name);

    // Write updated contacts to JSON file
    let file = File::create("contacts.json").expect("Failed to create file");
    serde_json::to_writer(file, &contacts).expect("Failed to write data");

    println!("\nContact deleted");

}

fn main() -> io::Result<()> {
    loop {    
        let menu = String::from(
    "\n                   Address Book
            --------------------------
            |      Select One:       |
            | 1. Add New Contact     |
            | 2. Edit Contact        |
            | 3. View Contact        |
            | 4. View All Contacts   |
            | 5. Delete Contact      |
            | 6. Quit                |
            --------------------------
            
            Pick one");

        println!("{}", menu);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let selection: u32 = input.trim().parse().expect("Please type a number: ");

        match selection {
            1 => create_contact(),
            2 => edit_contact(),
            3 => view_contact(),
            4 => view_all(),
            5 => delete(),
            6 => break,
            _ => println!("Invalid selection"),
            
            
        }
    }
    Ok(())
}
