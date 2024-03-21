use std::{
    fs::File,
    io::{self, ErrorKind, Read},
};

fn main() {
    // try to open a file,
    // if that succeeds, return it
    // else, if it was not found
    //   try and creat the file
    //   if that succeeds, return the new file
    //   else panic and end program
    // else, if something else went wrong, panic and end program
    let res = File::open("hello.txt");

    let _file = match res {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(err) => panic!("problem creating the file: {err}"),
            },
            other_err => {
                panic!("problem opening the file: {:?}", other_err);
            }
        },
    };

    // alternatively, get the value inside the Result::Ok, ignoring the Err and panicking if anything fails

    let _another_file =
        File::open("hello.txt").expect("my custom message for future me if this panics");
    // prefer .expect to File::open("hello.txt").unwrap();

    // create functions that return Results!
    let result = read_user_name_from_file();
    match result {
        Ok(user) => println!("it's {user}!"),
        Err(e) => panic!("something went wrong: {}", e),
    };

    let same_result = better_read_user_name_from_file();
    if let Ok(user) = same_result {
        println!("it's {user}!");
    };
}

fn read_user_name_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("users.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username.trim().to_string()),
        Err(e) => Err(e),
    }
}

// same as above but uses the much more succinct ? operator
//  ?  is a shortcut that can be used on functions that return a Result. when those Results wrap an Err, the whole function stop and returns that Err, when they succeed and wrap an Ok, that is returned or resolved in the function and we continue
fn better_read_user_name_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("users.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username.trim().to_string())
}
