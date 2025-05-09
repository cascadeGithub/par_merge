use std::fs::{self, File};
use std::io::Read;
use std::path::PathBuf;

fn main() {
    let base_string = read_file("base.par");
    let insert_string = read_file("insert.par");

    let output_string = create_output_file(&base_string, &insert_string);
    fs::write("output.par", output_string).expect("failed to write the output");
}
fn create_output_file(base_string: &str, insert_string: &str) -> String {
    let mut uani = collect_uani(insert_string);
    let mut uiso = collect_uiso(insert_string);

    let mut output_string = String::new();
    let lines: Vec<&str> = base_string.split("\n").collect();
    for line in lines {
        match line {
            line if line.starts_with("UANI") => {
                output_string.push_str(&uani.pop().expect("UANI line not found"));
                output_string.push('\n');
            }
            line if line.starts_with("UISO") => {
                output_string.push_str(&uiso.pop().expect("UISO line not found"));
                output_string.push('\n');
            }
            _ => {
                output_string.push_str(line);
                output_string.push('\n');
            }
        }
    }
    output_string
}

fn collect_uani(input: &str) -> Vec<String> {
    let mut lines: Vec<String> = input
        .split('\n')
        .filter(|line| line.starts_with("UANI"))
        .map(|line| line.to_owned())
        .collect();

    lines.reverse();
    lines
}

fn collect_uiso(input: &str) -> Vec<String> {
    let mut lines: Vec<String> = input
        .split('\n')
        .filter(|line| line.starts_with("UISO"))
        .map(|line| line.to_owned())
        .collect();

    lines.reverse();
    lines
}

fn read_file(path: &str) -> String {
    let input_path = PathBuf::from(path);
    let mut file = File::open(input_path).expect("file not found");
    let mut output = String::new();
    file.read_to_string(&mut output).expect("invalid contents");
    output
}
