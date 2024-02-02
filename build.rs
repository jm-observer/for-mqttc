use std::process::Command;

fn main() {
    //npx tailwindcss -i ./src-web/input.css -o ./src-web/output.css
    let status = Command::new("npx.cmd")
        .args([
            "tailwindcss",
            "-i",
            "./src-web/input.css",
            "-o",
            "./src-web/output.css",
        ])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
    if !status.success() {
        panic!()
    }
    tauri_build::build()
}
