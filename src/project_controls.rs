use std::path::PathBuf;
use std::process;
use std::fs;
use std::io::Write;
use std::env;
use std::str::FromStr;

use crate::Project;

pub fn get_projects(config_path: &mut PathBuf) -> Vec<Project>{
    config_path.pop();
    config_path.push("projects.conf");
    let mut projects = Vec::new();
    let projects_string = fs::read_to_string(config_path).expect("error reading projects file");
    let project_lines:Vec<&str> = projects_string.split("\n").collect();
    let mut first = 0;
    for line in project_lines{
        first = first + 1;
        if first != 1{
            if line.len() > 1{
                let settings: Vec<&str> = line.split(":").collect();
                let category = settings[0].to_owned();
                let entity = settings[1].to_owned();
                let project = settings[2].to_owned();
                let notes_string = settings[3].to_owned();
                let folder_string = settings[4].to_owned();
                let notes_folder = PathBuf::from_str(&notes_string.trim_end()).expect("error reading notes string");
                let project_folder = PathBuf::from_str(&folder_string.trim_end()).expect("error reading folding sering"); 
                let mut active = false;
                let boxname = settings[6].to_owned();
                if settings[5] == "yes"{
                    env::set_var("BOXNAME", boxname.clone());
                    active = true;
                }
                let new_project = Project{category: category, entity: entity, project_name: project, files_folder: project_folder, notes_folder: notes_folder,  active: active, id: first, boxname: boxname};
                println!("{} {} LOADED!", &new_project.entity, &new_project.project_name);
                projects.push(new_project);
            }
        }
    }
    return projects
}


pub fn switch_project(projects: &mut Vec<Project>){
    for project in projects.clone(){
        if project.active == false{
            println!("{} {}|{}", project.id, project.entity, project.project_name);
        }
    }
    println!("\nnew project selection?\n");
    let mut response = String::new();
    std::io::stdin().read_line(&mut response).unwrap();
    if response.len() > 1{
        let new_id:i32 = response.trim_end().parse().expect("error converting to i32");
        for project in projects{
            if project.id ==  new_id{
                project.active = true;
                println!("project found switching to {} {}", project.entity, project.project_name);
                env::set_var("BOXNAME", project.boxname.clone());
            }
            else if project.id != new_id{
                project.active = false;
            }
            else{
                println!("error unknown project id")
            }
        }
    }
    else{
        println!("error we need user input here dummy!");
    }
    
}

pub fn save_projects(projects: &Vec<Project>, config_path: &PathBuf){
   let mut save_file_path = config_path.clone();
    save_file_path.pop();
    save_file_path.push("projects.conf");
    let mut save_file = fs::File::create(save_file_path).expect("error creating save_file");
    save_file.write_all(b"category:entity:name:notes:folder:active:box_name\n").expect("error writing first line to file");
    for project in projects{
        let default = format!{"{}:{}:{}:{}:{}:", project.category, project.entity, project.project_name, project.notes_folder.display(),project.files_folder.display()};
        let mut _outline = String::new();
        if project.active{
            _outline = format!("{}yes:{}\n", default, project.boxname);
        }
        else{
            _outline = format!("{}no:{}\n", default, project.boxname);
        }
        save_file.write_all(_outline.as_bytes()).expect("error writing outline");
    }
}

pub fn new_project(projects: &mut Vec<Project>, project_dir: &PathBuf, notes_dir: &PathBuf, tools_dir: &PathBuf, boxtemplate: &String){
    let mut new_id = 0;
    for project in projects.clone(){
        if project.id > new_id{
            new_id = project.id + 1;
        }
    }
    let mut new_project_dir = project_dir.clone();
    let mut new_note_dir = notes_dir.clone();
    let mut existing_folders = String::new();
    let mut category_name = String::new();
    let mut entity_name = String::new();
    let mut project_name = String::new();
    println!("Category? [real, ctfs, other]");
    std::io::stdin().read_line(&mut category_name).unwrap();
    println!("entity name?");
    std::io::stdin().read_line(&mut entity_name).unwrap();
    println!("project name?");
    std::io::stdin().read_line(&mut project_name).unwrap();
    println!("do you have an existing notes and folder structure to copy over?\ny/n");
    std::io::stdin().read_line(&mut existing_folders).unwrap();
    let category_name = category_name.trim_end().to_owned();
    let entity_name = entity_name.trim_end().to_owned();
    let project_name = project_name.trim_end().to_owned();
    if existing_folders.contains("y") || existing_folders.contains("Y"){
        let mut files_to_copy = String::new();
        let mut notes_to_copy = String::new();
        println!("path to project folder folder to copy:");
        std::io::stdin().read_line(&mut files_to_copy).unwrap();
        println!("path to notes folder to copy:");
        std::io::stdin().read_line(&mut notes_to_copy).unwrap();
        files_to_copy.pop();
        notes_to_copy.pop();
        println!("files to copy: {}", files_to_copy);
        println!("notes to copy: {}", notes_to_copy);
        println!("files destination: {}", new_project_dir.display());
        println!("notes destination: {}", new_note_dir.display()); 
        let folder_move_success = process::Command::new("mv")
                                            .arg("-i")
                                            .arg(files_to_copy)
                                            .arg(new_project_dir.display().to_string())
                                            .status().expect("unable to call the system mv command");
        let note_move_success = process::Command::new("mv")
                                            .arg("-i")
                                            .arg(notes_to_copy)
                                            .arg(new_note_dir.display().to_string())
                                            .status().expect("unable to call the system mv command");
        if folder_move_success.success(){
            println!("we copied the project folder correctly!!");
        }
        else{
            println!("failed to copy the project folder, try to move it manually!");
        }
        if note_move_success.success(){
            println!("we copied the notes folder correctly!!");
        }
        else{
            println!("failed to copy the notes folder, try to move it manually!");
        }
        new_project_dir.push(&category_name);
        new_project_dir.push(&entity_name);
        new_note_dir.push(&category_name);
        new_note_dir.push(&entity_name);
        new_project_dir.push(&project_name);
        new_note_dir.push(&project_name);
     
    }
    else{
        new_project_dir.push(&category_name);
        new_project_dir.push(&entity_name);
        new_note_dir.push(&category_name);
        new_note_dir.push(&entity_name);
        new_project_dir.push(&project_name);
        new_note_dir.push(&project_name);
        fs::create_dir_all(&new_project_dir).expect("error creating new files folder");
        fs::create_dir_all(&new_note_dir).expect("error creating new notes folder");
    }
    let box_name = format!("atarchbox_{}", project_name);
    let mut box_name_path = new_project_dir.clone();
    box_name_path.push("boxname");
    let mut box_name_file = fs::File::create(box_name_path).expect("Error creating box name file");
    box_name_file.write_all(&box_name.as_bytes()).expect("error writing boxname to box file");
    let project_volume = format!("{}:/project:rw", new_project_dir.display());
    let toold_volume = format!("{}:/tools:rw", tools_dir.display());
    println!("distrobox create --root --clone {} --volume {} --volume {} --name {}", boxtemplate, toold_volume,  project_volume, box_name);
    let distrobox_result = process::Command::new("distrobox")
                                                                .arg("create")
                                                                .arg("--root")
                                                                .arg("--clone")
                                                                .arg(boxtemplate)
                                                                .arg("--volume")
                                                                .arg(toold_volume)
                                                                .arg("--volume")
                                                                .arg(project_volume)
                                                                .arg("--name")
                                                                .arg(&box_name)
                                                                .status()
                                                                .expect("error getting distrobox status");
    if distrobox_result.success(){
        println!("we made a distrobox oh boy!");
        let distrobox_start_result = process::Command::new("distrobox")
                                                                        .arg("enter")
                                                                        .arg("--root")
                                                                        .arg(&box_name)
                                                                        .arg("--")
                                                                        .arg("sudo")
                                                                        .arg("-s")
                                                                        .arg("ln")
                                                                        .arg("-sf")
                                                                        .arg("/project/boxname")
                                                                        .arg("/etc/boxname")
                                                                        .status()
                                                                        .expect("error getting response from distrobox start");
        if distrobox_start_result.success(){
            println!("distrobox was started as well!!!! good job me!");
        }
        else{
            println!("ooof did not start successfully try entering it yoruself");
        }
    }
    else{
        println!("ooof distrobox did not work.... try creating it yourself");
    }
    let new_project = Project{
                                    category: category_name.trim_end().to_owned(),
                                    entity: entity_name.trim_end().to_owned(), 
                                    project_name: project_name.trim_end().to_owned(), 
                                    notes_folder: new_note_dir, 
                                    files_folder:new_project_dir, 
                                    active: false,
                                    id: new_id,
                                    boxname: box_name,
                                      };
    projects.push(new_project);

}


pub fn remove_project(projects: &mut Vec<Project>){
    for project in projects.clone(){
        println!("{} {} {}", project.id, project.entity, project.project_name);
    }
    let mut project_to_remove = String::new();
    println!("project to remove?");
    let input_status = std::io::stdin().read_line(&mut project_to_remove);
    if input_status.is_err(){
        println!("we need input here dummy!");
        return;
    }
    input_status.unwrap();
    let mut project_to_keep = Vec::new();
    if project_to_remove.len() > 0{
        let remove_id: i32 = project_to_remove.trim_end().parse().unwrap();
        let mut project_set = false;
        for project in projects.clone(){
            if project.id == remove_id{
                println!("will remove {} {}", project.entity, project.project_name);
                project_set = true;
                let _distrobox_stop_status = process::Command::new("distrobox").arg("stop").arg("--root").arg(&project.boxname).status().expect("error stopping distrobox");
                let distrobox_rm_status = process::Command::new("distrobox")
                                                                        .arg("rm")
                                                                        .arg("--root")
                                                                        .arg("-f")
                                                                        .arg(&project.boxname)
                                                                        .status().expect("error calling distrobox");
                if distrobox_rm_status.success(){
                    println!("Distrobox Removal Successful!!!");
                }
                else{
                    println!("Distrobox Removal Failed, manual removal required!");
                }
            }
            else {
                println!("{} {} will be kept", project.entity, project.project_name);
                project_to_keep.push(project);
            }
        }
        if project_set{
            projects.clear();
            projects.append(&mut project_to_keep);
        }
        else{
            println!("error no prjects found to remove")
        }
        
    }
    else{
        println!("we need user in put here dummy!!");
    }
}