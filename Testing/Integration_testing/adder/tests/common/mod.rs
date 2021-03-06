use std::fs::File;
use std::path::Path;

pub fn setup() {
    // some setup code, like creating required files/directories, starting
    // servers, etc.
    let path = Path::new("setup.txt");
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };
}

