use crate::Project;
use std::path::PathBuf;
use std::process;

pub fn open_in_dolphin(folder: &str, project: Project){
    let mut to_open = PathBuf::new();
    match folder{
        "notes" => to_open.push(project.notes_folder),
        "files" => to_open.push(project.files_folder),
        _ => println!("unknown entry... this should literally be impossible... how.... how tf did you.... what")
    }
    process::Command::new("dolphin")
                         .arg(to_open)
                         .spawn().expect("error opening dolphin");
}

pub fn stop_all_boxes(projects: &Vec<Project>){
    let mut problem_childs: Vec<Project> = Vec::new();
    let mut all_done = true;
    for project in projects{
        let stopped = process::Command::new("distrobox")
                             .arg("stop")
                             .arg("--root")
                             .arg(&project.boxname)
                             .status().expect("error spawing distrobox");
        if stopped.success(){
            println!("{} sucessfully stopped!", &project.boxname);
        }
        else{
            println!("{} not stopped!!!!!!!", &project.boxname);
            all_done = false;
            problem_childs.push(project.clone());
        }
    }
    if all_done{
        println!("All boxes stopped GO US YAAAAA WE DID IT");
    }
    else{
        println!("OOOOOOOF some boxes didn't stop yo, that's cringe");
        println!("here are the problem childs, you may need to stop them manually");
        for child in problem_childs{
            println!("{}",child.boxname);
        }
    }
}



pub fn project_standalone_terminal(project: Project, mut terminal: String){
    println!("{}", terminal);
    terminal = terminal.trim_end().to_owned();
    let mut profile = false;
    if terminal.contains("profile"){
        profile = true;
    }
    let terminal_vec:Vec<&str> = terminal.split(" ").collect();
    let mut terminal_start = process::Command::new(terminal_vec[0]);
    let mut first = true;
    for arg in terminal_vec{
        if first == true{
            first = false;
        }
        else{
            terminal_start.arg(arg);
        }
    } 
    if profile == false {
        terminal_start.arg(&project.boxname);
    }
    println!("{}", terminal);
    let start = terminal_start.spawn();
    match start{
        Ok(_child) => println!("New Terminal Started, you can retun to the menu now!"),
        Err(_error) => println!("error starting new terminanl, something may be wrong with the terminal part of your config!\nCheck ~/.conf/hack_menu/conf for errors")
    }
    //process::Command::new("konsole").arg("--profile").arg("attack").arg(project.boxname).spawn().expect("error opeing konsole");
}


pub fn project_inline_terminal(project: Project){
    process::Command::new("distrobox").arg("enter").arg("--root").arg(project.boxname).status().expect("error opeing konsole");
}