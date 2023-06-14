use std::process::Command;

#[tauri::command]
pub fn on() {
  let _ = Command::new("gatttool")
    .arg("-b")
    .arg("be:89:f0:01:d7:b2")
    .arg("--char-write-req")
    .arg("-a")
    .arg("0x0009")
    .arg("-n")
    .arg("7e00040100000000ef")
    .spawn();   
}

#[tauri::command]
pub fn off() {
  let _ = Command::new("gatttool")
    .arg("-b")
    .arg("be:89:f0:01:d7:b2")
    .arg("--char-write-req")
    .arg("-a")
    .arg("0x0009")
    .arg("-n")
    .arg("7e0004000000ff00ef")
    .spawn();
}

#[tauri::command]
pub fn brightness(brightness: &str) {
    let brightness = match brightness {
        "low" => "7e00010a00000000ef",
        "medium" => "7e00013200000000ef",
        "high" => "7e00016400000000ef",
        _ => "7e00010000000000ef"
    };
    
    let _ = Command::new("gatttool")
        .arg("-b")
        .arg("be:89:f0:01:d7:b2")
        .arg("--char-write-req")
        .arg("-a")
        .arg("0x0009")
        .arg("-n")
        .arg(brightness)
        .spawn();
}

#[tauri::command]
pub fn color(color: &str) {
    let color = match color {
        "red" => "7e000503ff000000ef",
        "green" => "7e00050300ff0000ef",
        "blue" => "7e0005030000ff00ef",
        "yellow" => "7e000503ffff0000ef",
        "lightgreen" => "7e00050390ee9000ef",
        "lightblue" => "7e000503add8e6ffef",
        "orange" => "7e000503ffa50000ef",
        "purple" => "7e000503a020f000ef",
        "white" => "7e000503ffffff00ef",
        _ => "7e00010000000000ef"
    };
    
    let _ = Command::new("gatttool")
        .arg("-b")
        .arg("be:89:f0:01:d7:b2")
        .arg("--char-write-req")
        .arg("-a")
        .arg("0x0009")
        .arg("-n")
        .arg(color)
        .spawn();
}