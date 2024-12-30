use std::path::PathBuf;
use std::fs;
use directories::UserDirs;
mod menu;
mod project_controls;
mod box_controls;
mod install;

#[derive(Clone)]
struct Project{
    category: String,
    entity: String,
    project_name: String,
    notes_folder: PathBuf,
    files_folder: PathBuf,
    active: bool,
    boxname: String,
    id: i32,
} 

fn main() {
    print!("
    ⠀⠀⠀⣠⠶⠚⠛⠛⠛⠲⢦⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
    ⠀⠀⠀⣴⠟⠁⠀⠀⠀⠀⠀⠀⠀⠻⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
    ⠀⣠⣾⣷⣄⠀⠀⠀⢀⣠⣤⣤⡀⠀⢿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
    ⢸⣿⡿⢃⣸⡶⠂⢠⣿⣿⡿⠁⣱⠀⢸⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
    ⢸⡏⠉⠩⣏⣐⣦⠀⠛⠦⠴⠚⠁⠀⣸⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
    ⣼⠧⠶⠶⠶⠿⠶⠶⠖⠚⠛⠉⠁⠀⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⣰⠶⠶⡄⠀⠀
    ⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⠀⠀⠀⠀⠀⠀⠀⠀⢠⡟⠀⠀⢹⠀⠀
    ⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⢤⢠⡆⠀⢸⡄⠀⠀⠀⠀⠀⠀⢀⡿⠁⠀⠀⡾⠀⠀
    ⢹⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠈⡇⠀⠸⣧⣠⠴⠶⠖⠲⢶⡞⠁⠀⢈⡼⢃⠀⠀
    ⠸⡆⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀⡇⠀⠀⢿⠁⠄⣲⡶⠶⠿⢤⣄⡀⠛⢛⠉⢻⠀
    ⠀⢿⡀⠀⠀⠀⠀⠀⠀⠀⢸⠠⣇⠀⠀⠀⠀⠊⠁⠀⠀⠀⠀⠀⠙⢦⠈⠙⠓⣆
    ⠀⠈⢷⡀⠀⠀⠀⠀⠀⢠⠏⡀⣬⣹⣦⠀⠀⠀⠀⠀⠁⠀⠀⠀⠀⠈⡿⠶⠶⠋
    ⠀⠀⠈⢷⡀⠀⠀⠀⠀⠘⠛⠛⠋⠀⠀⠀⠀⠀⠀⠄⠀⠀⠀⠀⠀⣼⠃⠀⠀⠀
    ⠀⠀⠀⠀⠙⢦⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠄⠀⠀⣠⡞⠁⠀⠀⠀⠀
    ⠀⠀⠀⠀⠀⠀⠈⠛⣷⢶⣦⣤⣄⣀⣠⣤⣤⠀⣶⠶⠶⠶⠛⠁⠀⠀⠀⠀⠀⠀
    ⠀⠀⠀⠀⣀⡀⠀⣰⠇⣾⠀⠀⠈⣩⣥⣄⣿⠀⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
    ⠀⠀⠀⠀⢿⡉⠳⡟⣸⠃⠀⠀⠀⠘⢷⣌⠉⠀⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
    ⠀⠀⠀⠀⠀⠙⢦⣴⠏⠀⠀⠀⠀⠀⠀⠉⠳⠶⠏⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
    ");
    let user_dirs = UserDirs::new().expect("error getting user directories");
    let mut config_path = user_dirs.home_dir().to_path_buf();
    config_path.push(".config/pyro_hackmenu/conf");
    if config_path.as_path().exists() == false{
        install::install(&config_path);
    }
    let mut project_base_folder = PathBuf::new();
    let mut project_base_notes = PathBuf::new();
    let mut tools_folder = PathBuf::new();
    println!("config already generated\nloading config file...\n");
    let settings_string = fs::read_to_string(&config_path).expect("error reading config file");
    let settings: Vec<&str> = settings_string.split("\n").collect();
    let mut box_template = String::new();
    let mut terminal_cmd = String::new();
    let mut file_explorer_cmd = String::new();
    for line in settings{
        if line.len() > 1{
            let setting_vec: Vec<&str> = line.split(":").collect();
            match setting_vec[0]{
                "Project_files" => project_base_folder.push(setting_vec[1].trim_end()),
                "Project_notes" => project_base_notes.push(setting_vec[1].trim_end()),
                "tools_folder" => tools_folder.push(setting_vec[1].trim_end()),
                "box_template" => box_template = setting_vec[1].trim_end().to_owned(),
                "terminal_cmd" => terminal_cmd = setting_vec[1].trim_end().to_owned(),
                "file_explorer_cmd" => file_explorer_cmd = setting_vec[1].trim_end().to_owned(),
                _ => println!("error unknown setting: {}", setting_vec[0])
            }
        }
    }
    print!("
    Project Folders: {} 
    Note Folders: {}
    Tools Folder: {}
    distrobox template: {}\n
", project_base_folder.display(), project_base_notes.display(), tools_folder.display(), box_template);
    println!("loading project configs...");
    let projects = project_controls::get_projects(&mut config_path);
    println!("Enter to start main menu");
    let mut enter = String::new();
    std::io::stdin().read_line(&mut enter).unwrap();
    menu::main_menu(projects, &config_path, &project_base_folder, &project_base_notes, &tools_folder, box_template, terminal_cmd, file_explorer_cmd);
}
