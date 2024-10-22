use std::{fs, process::Command};
use serde_yaml;
use serde::Deserialize;
use glob::glob;

#[derive(Debug, Deserialize)]
struct Config {
    cc: String,
    src: Vec<String>,
    output: String,
    std: Option<String>,
    flags: Option<Vec<String>>,
    extra_libs: Option<Vec<String>>,
    extra_incl_paths: Option<Vec<String>>,
}

fn main() {
    // Read the YAML file
    let yaml_str = fs::read_to_string("ezc.yml")
        .expect("Failed to read the file");

    // Parse the YAML into the Config struct
    let config: Config = serde_yaml::from_str(&yaml_str)
        .expect("Failed to parse YAML");

    /*
    // Output the parsed configuration
    println!("Compiler: {}", config.cc);
    println!("Source Files: ");
    */
    // Process src patterns
    let mut source_files = Vec::new(); // Collect matched files
    for pattern in config.src {
        //println!("Processing pattern: {}", pattern);
        if pattern.contains('*') || pattern.contains('{') {
            // Use glob to expand patterns
            match glob(&pattern) {
                Ok(paths) => {
                    for path in paths {
                        match path {
                            Ok(path_str) => {
                                //println!("Matched file: {}", path_str.display());
                                source_files.push(path_str.display().to_string());
                            },
                            Err(e) => println!("Error matching path: {}", e),
                        }
                    }
                }
                Err(e) => println!("Error in glob pattern: {}", e),
            }
        } else {
            //println!("Matched file: {}", pattern);
            source_files.push(pattern);
        }
    }
    
    // Create the compilation command
    let mut compile_command = config.cc.to_string();
    compile_command += " -c";
    for file in &source_files { compile_command += format!(" {}", file).as_str(); }
    if config.std.is_some() { compile_command += format!(" -std={}", config.std.unwrap()).as_str(); }
    if config.flags.is_some() {
        for flag in config.flags.unwrap() { compile_command += format!(" {}", flag.as_str()).as_str(); }
    }
    if config.extra_incl_paths.is_some() {
        for incl_path in config.extra_incl_paths.unwrap() {
            compile_command += format!(" -I{incl_path}").as_str();
        }
    }

    // Print the final command (or execute it, if needed)
    //println!("1st Compilation command: {}", compile_command);
    let output = Command::new("sh")
        .arg("-c")
        .arg(compile_command)  // Your compiled command string
        .output()
        .expect("Failed to execute 1st compile command");

    if !output.status.success() {
        println!("First compilation command failed D:\nHere's the output:\n{}", std::str::from_utf8(&output.stderr).unwrap());
        return;
    }

    compile_command = config.cc;
    compile_command += format!(" -o {}", config.output).as_str();
    for lib in config.extra_libs.unwrap() {
        compile_command += format!(" -l{lib}").as_str();
    }
    compile_command += " *.o";

    //println!("2nd Compilation command: {}", compile_command);
    let output = Command::new("sh")
        .arg("-c")
        .arg(compile_command)  // Your compiled command string
        .output()
        .expect("Failed to execute 2nd compile command");

    if !output.status.success() {
        println!("Second compilation command failed D:");
        return;
    }

    println!("Done!");
}
