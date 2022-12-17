use std::env;
use std::fs;
use std::io;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        exit_with_message("Please enter a name.")
    }
    let base_path: String = args[1].to_owned();
    let bin_path: String = base_path.to_owned() + "/bin";
    let mut src_path: String = base_path.to_owned() + "/src";
    let header_path: String = base_path.to_owned() + "/header";
    _ = make_project_dir(&base_path);
    _ = make_project_dir(&bin_path);
    src_path = make_project_dir(&src_path);
    _ = make_project_dir(&header_path);

    let mut file_path = src_path.to_owned();

    file_path.push_str("/main.cpp");
    println!("main file: {}",&file_path);
    _ = fs::write(&file_path, "int main() {\n    cout << \"Hello World\"\n}");

    file_path = base_path.to_owned();
    file_path.push_str("/README.md");
    _ = fs::write(file_path, "");
    
}

fn exit_with_message(message: &str) {
    print!("{}",message);
    process::exit(1);
}

fn make_project_dir(name: &str) -> String {
    let mut filepath: String = env::current_dir().expect("there should always be a filepath").to_str().unwrap().to_owned();
    filepath.push_str("/");
    filepath.push_str(&name.to_owned());
    let result = fs::create_dir(&filepath);

    match result {
        Ok(_) => {},
        Err(why) if why.kind() == io::ErrorKind::AlreadyExists => exit_with_message("A directory with this name already exists!"),
        Err(why) if why.kind() == io::ErrorKind::PermissionDenied => exit_with_message("You do not have the proper priviliges."),
        Err(_) => panic!("Something went wrong when creating folder!)")
    }
    return filepath;
}
