use std::process::Command;

fn main() {
    let output = Command::new("sensors")
        .output()
        .expect("sensors コマンド実行失敗");

    let text = String::from_utf8_lossy(&output.stdout);

    println!("=== CPU Temperature Monitor ===");

    for line in text.lines() {
        if line.contains("Package id")
            || line.contains("Core ")
            || line.contains("temp1")
        {
            println!("{}", line.trim());
        }
    }
}