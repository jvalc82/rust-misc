use std::env;
use regex::Regex;
extern crate regex;


fn main() -> Result<(), String> {
    let cmd_args: Vec<String> = env::args().collect();

    if cmd_args.len() != 2 {
        return Err(format!("Usage is: {} [url]", cmd_args[0]));
    }
    
    let regex: Regex = Regex::new(r"(?x)
        ^(?:(?P<proto>https?|file|ftp|sftp)?:?/{2}
        (?P<domain>/?[[:alnum:][:punct:]]+)\.(?P<dext>com|org|ru|net|sh|py|txt|c|go[vb])
        (?P<path>[[:alnum:][:punct:]]*))?$"
    ).expect("Could not compile regex.");

    if let Some(captures) = regex.captures(cmd_args[1].as_str()) {
        for group_name in regex.capture_names() {
            if let Some(group_name) = group_name {
                if let Some(value) = captures.name(group_name) {
                    println!("{}: {}", group_name, value.as_str());
                }
            }
        }
    }

    Ok(())
}
