use std::process::Command;
use std::{thread, time::Duration};

fn main() {
    loop {
        let output = Command::new("sensors")
            .output()
            .expect("sensors を実行できません");

        let result = String::from_utf8_lossy(&output.stdout);

        print!("\x1B[2J\x1B[1;1H"); // 画面クリア

        for line in result.lines() {
            if line.contains("temp1") {
                println!("CPU温度監視");
                println!("----------------");
                println!("{}", line);
            }
        }

        thread::sleep(Duration::from_secs(1));
    }
}