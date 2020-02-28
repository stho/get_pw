
extern crate keyring;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let service = "my_application_name";
    let username = "ee_username";

    let keyring = keyring::Keyring::new(&service, &username);

    let password = "topS3cr3tP4$$w0rd";
    keyring.set_password(&password)?;

    let password = keyring.get_password()?;
    println!("The password is '{}'", password);

    Ok(())
}


