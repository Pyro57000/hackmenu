use std::env;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process;
use std::str::FromStr;
use clearscreen::clear;
use directories::UserDirs;
use clearscreen;

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



fn install(config_path: &PathBuf){
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
    println!("path to save project notes?");
    std::io::stdin().read_line(&mut notes_response).unwrap();
    println!("path to save project files?");
    std::io::stdin().read_line(&mut files_response).unwrap();
    println!("path to folder with your custom tools?");
    std::io::stdin().read_line(&mut tools_response).unwrap();
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
    let config_string = format!("Project_files:{}\nProject_notes:{}\ntools_folder:{}\nbox_template:{}", files_response.trim_end(), notes_response.trim_end(), tools_response.trim_end(),template_name.trim_end());
    config_file.write_all(config_string.as_bytes()).expect("error writing to config file");
    let default_projectline = format!("default:default:default:{}:{}:yes:{}", &notes_response.trim_end(), &files_response.trim_end(), &template_name.trim_end());
    projects_conf_file.write_all(default_projectline.as_bytes()).expect("error writing default project line");
    println!("config file generated and saved to {}\n", config_path.display());
    println!("please make sure to install distrobox:\nhttps://github.com/89luca89/distrobox\n\nthis will require either docker or podman as well.\n\n");
    println!("please rerun the program");
    std::process::exit(1);
}


fn get_projects(config_path: &mut PathBuf) -> Vec<Project>{
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
                    env::set_var("CURRENT_PROJECT_BOX", boxname.clone());
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


fn switch_project(projects: &mut Vec<Project>){
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
                env::set_var("CURRENT_PROJECT_BOX", project.boxname.clone());
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

fn save_projects(projects: &Vec<Project>, config_path: &PathBuf){
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


fn get_active_project(projects: &Vec<Project>) -> &Project{
    let mut active_project = &projects[0];
    for project in projects{
        if project.active == true{
            active_project = project
        }
    }
    return active_project
}

fn new_project(projects: &mut Vec<Project>, project_dir: &PathBuf, notes_dir: &PathBuf, tools_dir: &PathBuf, boxtemplate: &String){
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
    let box_name = format!("atarchbox_{}", entity_name);
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


fn remove_project(projects: &mut Vec<Project>){
    for project in projects.clone(){
        println!("{} {} {}", project.id, project.entity, project.project_name);
    }
    let mut project_to_remove = String::new();
    println!("project to remove?");
    std::io::stdin().read_line(&mut project_to_remove).unwrap();
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
fn open_in_dolphin(folder: &str, project: Project){
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

fn stop_all_boxes(projects: &Vec<Project>){
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



fn project_standalone_terminal(project: Project){
    process::Command::new("konsole").arg("--profile").arg("ctfs").arg(project.boxname).spawn().expect("error opeing konsole");
}


fn project_inline_terminal(project: Project){
    process::Command::new("distrobox").arg("enter").arg("--root").arg(project.boxname).status().expect("error opeing konsole");
}


fn main_menu(mut projects: Vec<Project>, config_path: &PathBuf, base_files: &PathBuf, base_notes: &PathBuf, tools_dir: &PathBuf, boxtemplate: String){
    let mut loopize = true;
    loop {
        let active_project = get_active_project(&projects);
        let mut response = String::new();
        clear().expect("error clearing screen");
        print!("
,,,;;::ccccc::;;;::c::;,;::cccccllc::::::;:::;;;;,,;,'',,;,,;;;;;;;:;;;;;,,,,,,,,,,,'''''',,,,,,''''
,;;;::ccccc::::::ccc:;;;:ccccccclc::ccccccc::;;;;;;;;;;,,;;;;;;;;;;;;;;;,,,,,,,,,,,'''''''''',,,,,''
,;;:::ccc:cc:::ccc:::::::ccccclcccllccccllc::::::;;;;;;;;;;;;;;;;;;;;;;,,,,,,,,,,,''''''''...'',,,,'
,;;:::c::ccc::cc::::::::cclollllllolllllccccc::cc:::::;;;;;;;;;;;;;;;;;;,,,,,,,,,,'''''''''''..'',,,
,;::::::ccc::cc::::::ccloodollooooollllcccccc:llc::::::;;;;;;:;;;;;;;;;;;,,,,,,,,,,''''''''''''''',,
,;:::::c:::c::::ccccloddxxddxxddddodollllccclclcccccc:::::::::::::::;;;;;;;;,,,,,,,,,'''''''''''''',
;;:::::::c::c::clllodxxO0OKX0kkOkkxxxxxdooooolcccccccc:::::::cllc::::::::;;;;;,,,,,,,,,,'''''''''',,
;:::::c:cc::cclolclokO0KXNNX00KKK0O0KOxdxxdooccccclllccccccccdkdlcccccllcc::;;;;;;;;,,,,,,,,,,,',,,,
::::::cc::::coxdlllok00KNWNXXX0KXKOKNOkO0kddocccllllllccccclx0Kkodddoodddoollc::;;;;;;;;;,,,,,,,,,,,
:::::::c:::clkkodooxxkO0KX0xookKKkkKNKO0KkdoodolcllllollolldKNNXKKXKKKKKK0Okxdocc:cc:::;;;;;,,,,,,,,
::cc::cc::cldxllolodxxdoddc'.,okkxOXXOdxkxolkOdlllllllldkdokXNNNNNNNNX0kxollcc:::::cclc::;;;;;;,,,,,
:::::::cccldko:,.';cc:;:;....;clllOXOxxOkocoK0xooddollx0Odd0XNNNNNX0Oxdolcccc::;;;;;;:cllc:;;:;,,,,,
;;::c:::ccldkl;'...,''''....',;,';dxdkkdc;cONKxxOOOxddOXOdx0XXNNWNOdddxkOOOOkdllc:;,,,;cool:;;;;;,,;
,;;::;;::llco:,'..''..,.......''.';:ldl;,,:xXNOOXXX0xdkOOddkXNNWWWX00KXNNX0kxddddol:,''';lol:;;:;;,;
,,,;;;;;:coc;;'..;;. .,,;'.......':dxdc;ldc,l00xkXNKxodkkkkk0XNWWMWWWNXKOxdooolooool:;'..,lol::::;;;
',,,;;;;:cllc;,..',.  ','.  .....;odoo:;co:.'ldldOOx::x0KXX0kk0XNWWWXOxdoooollllllllcc:'..':lc:::;;;
',,,;;;;;:cccc:,. .          ..;cccccc:,,''.',,:l:;;:oOXXKOOOkxOXNNNXOxddooooollllllllc,....:c:::;;;
''',,,;;;;;;;cll,..   ..    .':lc:c:;,,......,,;:;;:cokkxxO00O0KXNNN0kxkkkxddoollllllll:'...':::::::
.''',,,,,,,,,;:c:,.. ..'.  ..','',;;'........',,;:;:::cdxxxddkKXXXKKKKXXXXXX0kdoloolllol;....,;:::::
..'''',,'',,,;;:::;..... ............... .'.....,;,',:ldc;:ldOKKK00KNWWWNNXK0xoooodooooo:'...';;;:;;
....'''''',,;;::cll:,''......  .      ..........'...,;;l:,,;oddkOOKNWWWWNX0kdodxxxxdddooc,...',;;;;,
......''''',;::cloodddol;.               ...........',.;;,,',:cxdd0KXXKKKXKOkxxkkkxdddooc,...';;,,,,
........''',;:clloddxxdo:'.              .. ...........''.'',;c:;cccodddk0KX0OOkxddddddo:...';;;;,,'
..........',;:cclodxxkxdl:,..          ...  ......'....'..':c,..'.,c,,,.,cxkO00Okxdddddc'..';:;;;,,'
..........',;;:cloodxkkkdol:'.    .  ...    ...... ......';c'.'...:;',;,'..,lxO00Oxxxo:'...,::;;,,,,
...........',;;:clodxkOOkxdol;.   ..  ..        ... ....',::'.''.',.........'oxdxxxdl;...';::;;;,,''
............',;:clodxkkOOOxddo;.   ......       ........',,',................';:clc;,...';::;;,,,'''
............',;:cldxkkOkkkxdddo;.    .....      .........,'...........'''','.',,'''....,cc:;;,,'''..
.............';:cldxxkkkkxddddxl,.   ....       .;c;'...................',;;cc;'...';clolc:;,,'''...
............'';clodxkkkkkxddddddl'    ...       .:lc;'................. ....',,''';lxkxdlc:;,'''....
........',,;:;coddxkOOOOOkxxddddd:.   ...     ..,''..................      . ..;cdkkkkxoc:;,'''.....
......',;::cllodxkkOOOOOOkxxxddddc.  ...      ..,;,'................... ..   .':odO0Okdl:;,'''......
.....',;:cloddxxkOOOOOOOkkxxdoooo;.  ..       .........................      .';cokOOxlc:;,''.......
....,;:clodxxkkOOOkO0OOOOxdlcc;;,......      .';,.................         ...',:ldxxxdlc;,''.......
...,:clodooxkkkO0OxO00OOxo:;;,. ........   .''.......... ..  ..           ..,,,;:codxxdlc:;,'.......
'',;clodolokOkxkOkkO00Oko:;;;.  ..... ..   .,,........'. ..  ..  ..   ..........;:codocclc:,,'......
              ___       __   __         ___               __        ___  __  
        |  | |__  |    /  ` /  \\  |\\/| |__     |__|  /\\  /  ` |__/ |__  |__) 
        |/\\| |___ |___ \\__, \\__/  |  | |___    |  | /~~\\ \\__, |  \\ |___ |  \\ 
                                                                             
         __   ___ ___     __                    __                           
        / _` |__   |     |__) |  | |\\ | | |\\ | / _`                          
        \\__> |___  |     |    |/\\| | \\| | | \\| \\__>     
    


NOTE SAVE PROJECT INFO BEFORE STOPPING THE APPLICATION, OR HOUR TRACKIGN WON'T BE ACCURATE
NOTE OPTION 10 WILL SAVE YOUR PROJECTS BEFORE QUITTING

Current Project: {} {}

        Main Menu:
            1 .) Show Active Project
            2 .) List Projects
            3 .) Switch Active Project
            4 .) Save Project Information
            5 .) Import New Project - and setup new Distrobox
            6 .) Remove Project
            7 .) Open A New Terminal in Current Active Project
            8 .) Open A Terminal In this windows for the current active project
            9 .) Open Project Files Folder In Dolphin
            10.) Open Project Notes Folder In Dolphin
            11.) Stop All Distroboxes
            12.) Quit Application
\n", active_project.entity, active_project.project_name);
        std::io::stdin().read_line(&mut response).expect("error getting menu input");
        clear().expect("error clearing screen");
        match response.as_str().trim_end(){
            "1" => println!("\n{} {}", active_project.entity ,active_project.project_name),
            "2" => {println!("+++++++++++++++++++++"); 
                    for project in &projects{
                        println!("++{}|{}++",project.entity ,project.project_name)}
                    println!("++++++++++++++++++++")},
            "3" => switch_project(&mut projects),
            "4" => save_projects(&projects, &config_path),
            "5" => new_project(&mut projects, &base_files, &base_notes, &tools_dir, &boxtemplate),
            "6" => remove_project(&mut projects),
            "7" => project_standalone_terminal(active_project.clone()),
            "8" => project_inline_terminal(active_project.clone()),
            "9" => open_in_dolphin("files", active_project.clone()),
            "10" => open_in_dolphin("notes", active_project.clone()),
            "11" => stop_all_boxes(&projects),
            "12" => {save_projects(&projects, &config_path);
                    let mut stop = String::new(); 
                    println!("stop all boxes?\ny/n");
                    std::io::stdin().read_line(&mut stop).unwrap();
                    if stop.contains("y"){
                        stop_all_boxes(&projects);
                    }
                    loopize = false},
            _ => println!("uknonwn selection")
        }
        if loopize == false{
            break
        }
        println!("\n\n\npress enter to return to the menu");
        let mut enter = String::new();
        std::io::stdin().read_line(&mut enter).unwrap();
    }
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
        install(&config_path);
    }
    let mut project_base_folder = PathBuf::new();
    let mut project_base_notes = PathBuf::new();
    let mut tools_folder = PathBuf::new();
    println!("config already generated\nloading config file...\n");
    let settings_string = fs::read_to_string(&config_path).expect("error reading config file");
    let settings: Vec<&str> = settings_string.split("\n").collect();
    let mut box_template = String::new();
    for line in settings{
        if line.len() > 1{
            let setting_vec: Vec<&str> = line.split(":").collect();
            match setting_vec[0]{
                "Project_files" => project_base_folder.push(setting_vec[1].trim_end()),
                "Project_notes" => project_base_notes.push(setting_vec[1].trim_end()),
                "tools_folder" => tools_folder.push(setting_vec[1].trim_end()),
                "box_template" => box_template = setting_vec[1].trim_end().to_owned(),
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
    let projects = get_projects(&mut config_path);
    println!("Enter to start main menu");
    let mut enter = String::new();
    std::io::stdin().read_line(&mut enter).unwrap();
    main_menu(projects, &config_path, &project_base_folder, &project_base_notes, &tools_folder, box_template);
}
