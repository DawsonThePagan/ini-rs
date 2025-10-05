# ini-rs

[![Rust](https://github.com/DawsonThePagan/ini-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/DawsonThePagan/ini-rs/actions/workflows/rust.yml)

A rust crate to read an INI file into a structure. The data can be accessed directly if required.

## Examples

Load an INI file

```Rust
use ini_rs::Ini;

// Load foo.ini
let mut foo = match Ini::new(r".\foo.ini".to_string()) {
    Ok(ini) => ini,
    Err(e) => panic!("{}", e),
};
```

Read data from an INI file
```Rust
match foo.get("foo", "foo") {
    Some(x) => {
        println!("{}", x)
    }
    None => {
        panic!("Key not found in section")
    },
}
```

Change data in the INI file, then save the change
```Rust
foo.set("bar", "bar", "bar");

match foo.save() {
    Ok(_) => println!("File saved successfully"),
    Err(e) => panic!("{}", e)
}
```

Remove data from the INI file
```Rust
foo.remove("bar", "bar");
foo.remove_section("foo");
```

## Functions

### new(location: String) -> Result<Ini, io::Error>
Load an INI file. If the file doesn't exist, create a blank Ini structure.
Will return Err(io::Error) if the file provided is invalid.

### set(section: &str, key: &str, value: &str) -> ()
Set, or create if it doesn't exist, a value in a section.
It will also create the section if the section doesn't exist.
This does not save the file.

### get(section: &str, key: &str) -> Option<String>
Get the key from the provided section.
If it doesn't exist, returns None.

### remove(section: &str, key: &str) -> ()
Remove a key from a section. Will not error if it doesn't exist.
This does not save the file.

### remove_section(section: &str) -> ()
Remove a section, will remove all keys from the section. Will not error if it doesn't exist.
This does not save the file.

### save() -> Result<usize, io::Error>
Save the changes to the file. Will not keep any comments present in the file.
Ok(usize) contains the new size of the file.

### from_string(str: String) -> Result<Ini, io::Error>
Make an INI structure from a string. Does not set the config_file so cannot save unless set manually.

### to_string() -> Result<String, io::Error>
Dump out the contents of the structure in the INI format to a string.
