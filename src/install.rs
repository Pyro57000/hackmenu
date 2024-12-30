use std::path::PathBuf;
use std::fs;
use std::io::Write;
use std::process;

pub fn install(config_path: &PathBuf){
    println!("No configpath detected!!! running install routines");
    let mut config_folder_path: PathBuf = config_path.clone();
    config_folder_path.pop();
    let del_on_fail = config_folder_path.clone();
    let mut projects_conf_path = config_folder_path.clone();
    projects_conf_path.push("projects.conf");
    fs::create_dir_all(config_folder_path).expect("error creating config dir");
    let mut config_file = fs::File::create(config_path).expect("error creating file");
    let mut projects_conf_file = fs::File::create(projects_conf_path).expect("error creating projects config file");
    projects_conf_file.write_all(b"category:entity:name:notes:folder:active:box_name\n").expect("error writing default project info");
    let mut notes_response = String::new();
    let mut files_response = String::new();
    let mut tools_response = String::new();
    let mut have_template = String::new();
    let mut template_name = String::new();
    let mut terminal_cmd = String::new();
    let mut file_explorer_cmd = String::new();
    println!("path to save project notes?");
    std::io::stdin().read_line(&mut notes_response).unwrap();
    println!("path to save project files?");
    std::io::stdin().read_line(&mut files_response).unwrap();
    println!("path to folder with your custom tools?");
    std::io::stdin().read_line(&mut tools_response).unwrap();
    println!("command to launch your terminal emulator to launch distrobox?");
    println!("for example konsole would be: konsole -e distrobox enter --root");
    std::io::stdin().read_line(&mut terminal_cmd).unwrap();
    println!("command to launch your file explorer to a specific folder");
    println!("for example dolphine is: dolphin ");
    std::io::stdin().read_line(&mut file_explorer_cmd).unwrap();
    print!("
This tool is mainly to handle distrobox creation and usage.
It's expecting you to have a distrobox that you will use as a template.
Do you have a distrobox set up to function as your template for all new projects?
");
    std::io::stdin().read_line(&mut have_template).unwrap();
    if have_template.contains("n"){
        println!("please set up a distrobox with root as a template and re-run this tool");
        println!("example distrobox setup command:");
        println!("distrobox create --root --image archlinux --name template");
        println!("then enter that distrobox and install all the tools you want and do what ever setup you need");
        println!("and re-run this tool.");
        process::Command::new("rm").arg(del_on_fail).spawn().expect("ERROR deleting config folder, please manually clean up");
        std::process::exit(1);
    }
    let _list = process::Command::new("distrobox").arg("list").arg("--root").status();
    println!("distrobox template name?");
    std::io::stdin().read_line(&mut template_name).unwrap();
    let config_string = format!("Project_files:{}\nProject_notes:{}\ntools_folder:{}\nbox_template:{}\nterminal_cmd:{}\nfile_explorer_cmd:{}", files_response.trim_end(), notes_response.trim_end(), tools_response.trim_end(),template_name.trim_end(),terminal_cmd.trim_end(),file_explorer_cmd.trim_end());
    config_file.write_all(config_string.as_bytes()).expect("error writing to config file");
    let default_projectline = format!("default:default:default:{}:{}:yes:{}", &notes_response.trim_end(), &files_response.trim_end(), &template_name.trim_end());
    projects_conf_file.write_all(default_projectline.as_bytes()).expect("error writing default project line");
    println!("config file generated and saved to {}\n", config_path.display());
    println!("please make sure to install distrobox:\nhttps://github.com/89luca89/distrobox\n\nthis will require either docker or podman as well.\n\n");
    println!("please rerun the program");
    std::process::exit(1);
}