extern crate git2;

use git2::Repository;
use std::env;
use std::process::{exit, Command};
use std::fs::copy;
use std::path::Path;
use std::string::String;


#[cfg(target_os = "linux")]
const VPK: &'static str = "bin/vpk_linux32";
#[cfg(target_os = "windows")]
const VPK: &'static str = "bin/vpk.exe";

fn main() {
    let hud = env::args().nth(1).expect("Error - invalid hud location");
    
    // help message
    if hud == "-h".to_string() {
        println!("
update-hud - updates and installs your tf2 hud using git repositories (e.g. github)
Usage:
update-hud [path to hud folder] [path to tf2 installation]
Note: point this toward the \"steamapps/common/Team Fortress 2\" directory");
        exit(0);
    } else {
        let repo = Repository::open(&hud).unwrap();
        repo.find_remote("origin").unwrap().fetch(&["origin"], None, None).unwrap();
    }

    let installation = env::args().nth(2).expect("Error - input tf2 installation directory");
    let mut vpk = installation.clone();
    vpk.push_str(VPK);
    //println!("{:?}", vpk);

    let mut vpk = Command::new(vpk);
    
    // necessary for vpk to run on linux - not necessary for windows
    if cfg!(target_os = "linux") {
        let mut ld_path = Path::new(&installation)
            .join("bin")
            .into_os_string();
        ld_path.push(":LD_LIBRARY_PATH");
        vpk.env("LD_LIBRARY_PATH", ld_path);
    }
    let output = vpk.arg(&hud).output().expect("Unable to run vpk");
    println!("{:?}", output);

    // since vpk writes .vpk files to the same directory as the folder, we need to copy it to
    // tf/custom.
    let mut custom_dir = installation.clone();
    custom_dir.push_str("tf/custom/");

    let mut vpk_file = String::from(Path::new(&hud)
                                    .file_name().unwrap()
                                    .to_str().unwrap());
    vpk_file.push_str(".vpk");
    let parent = Path::new(&hud).parent().unwrap();
    let parent = parent.canonicalize().unwrap();
    

    //println!("{}", vpk_file);
    //println!("{}", parent.to_str().unwrap());
    let mut file = String::from(parent.to_str().unwrap());
    file.push('/');
    file.push_str(&vpk_file);
    println!("{}", file);

    custom_dir.push_str(&vpk_file);

    copy(file, custom_dir).expect("Unable to copy file");
}

