#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(unused_must_use)]

use std::os::windows::process::CommandExt;


pub fn main() -> std::io::Result<()>
{
    let file = davids_standard_library::file::read_lines("devices");
    
    match file {
        Err(_) => {
            let _ = std::fs::write("devices", include_bytes!("../assets/devices"));

            let lines: Vec<String> = include_str!("../assets/devices")
                .lines()
                .map(String::from)  
                .collect();

            parse_lines(lines);
        },
        Ok(lines) => {
            parse_lines(lines);
        },
    }
        
    Ok(())

}

fn parse_lines(lines: Vec<String>)
{
    for vendor_id in lines{
        #[cfg(debug_assertions)]
        println!("{}", vendor_id);
        
        if let Ok(found) = is_match(vendor_id)
        {
            #[cfg(debug_assertions)]
            println!("{}", found);

            if found
            {
                #[cfg(debug_assertions)]
                println!("Running adb");

                let _ = std::process::Command::new("adb.exe")
                .args(["shell", "sh", "/sdcard/Android/data/moe.shizuku.privileged.api/start.sh"])
                .creation_flags(0x08000000)
                .output();
                
                std::thread::sleep(std::time::Duration::from_millis(500));
            }
        }
        
    }
}


fn is_match(vendor_id: String) -> rusb::Result<bool>
{
    for device in rusb::devices()?.iter()
    {
        let descriptor = device.device_descriptor()?;
        #[cfg(debug_assertions)]
        println!("vendor: {}, descriptor: {:04X}", vendor_id, descriptor.vendor_id());

        if format!("{:04X}", descriptor.vendor_id()) == vendor_id.to_ascii_uppercase()
        {
            return Ok(true);
        }
    }

    Ok(false)
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn adb_test() -> Result<(), std::io::Error> {
        
        let out = std::process::Command::new("adb.exe")
            .args(["shell", "sh", "/sdcard/Android/data/moe.shizuku.privileged.api/start.sh"])
            .creation_flags(0x08000000)
            .stdout(std::process::Stdio::piped())
            .output()?.stdout;

        let out = String::from_utf8(out);
        
        match out {
            Ok(out) => println!("{}", out),
            Err(_) => println!("err"),
        }
        Ok(())
    }

    #[test]
    fn single_run()
    {
        let _ = davids_standard_library::env::set_exe_dir();
        let res = main();
        match res {
            Ok(_) => println!("ok"),
            Err(_) => println!("error"),
        }
    }

    #[test]
    fn assets()
    {
        let test: Vec<String> = include_str!("../assets/devices")
                .lines()
                .map(String::from)  
                .collect();

        for line in test
        {
            println!("{}", line)
        }
    }



}