use std::path::PathBuf;
use clearscreen::clear;
use std::env;
use crate::project_controls;
use crate::box_controls;
use crate::Project;


fn get_active_project(projects: &Vec<Project>) -> &Project{
    let mut active_project = &projects[0];
    let mut already_set = false;
    for project in projects{
        if already_set == false{
            if project.active == true{
                active_project = project;
                already_set = true;
                env::set_var("BOXNAME", project.boxname.clone());
            }
        }
    }
    return active_project
}

pub fn main_menu(mut projects: Vec<Project>, config_path: &PathBuf, base_files: &PathBuf, base_notes: &PathBuf, tools_dir: &PathBuf, boxtemplate: String, terminal_cmd: String, file_explorer_cmd: String){
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
Terminal CMD: {}
Notes Directory: {}
Project Files: {}
Boxname: {}

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
\n",active_project.entity, active_project.project_name, terminal_cmd, active_project.notes_folder.display(), active_project.files_folder.display(), env::var("BOXNAME").unwrap());
        std::io::stdin().read_line(&mut response).expect("error getting menu input");
        clear().expect("error clearing screen");
        match response.as_str().trim_end(){
            "1" => print!("\nCATEGORY:{}\nENTITY:{}\nPROJECT NAME:{}\nNOTES FOLDER:{}\nPROJECT FOLDER:{}\nBOXNAME{},\nCURRENT BOXNAME VAR{}", active_project.category ,active_project.entity, active_project.project_name, active_project.notes_folder.display(), active_project.files_folder.display(), active_project.boxname, env::var("BOXNAME").unwrap()),
            "2" => {println!("+++++++++++++++++++++"); 
                    for project in &projects{
                        println!("++{}|{}++",project.entity ,project.project_name)}
                    println!("++++++++++++++++++++")},
            "3" => project_controls::switch_project(&mut projects),
            "4" => project_controls::save_projects(&projects, &config_path),
            "5" => project_controls::new_project(&mut projects, &base_files, &base_notes, &tools_dir, &boxtemplate),
            "6" => project_controls::remove_project(&mut projects),
            "7" => box_controls::project_standalone_terminal(active_project.clone(), terminal_cmd.to_owned()),
            "8" => box_controls::project_inline_terminal(active_project.clone()),
            "9" => box_controls::open_in_dolphin("files", active_project.clone()),
            "10" => box_controls::open_in_dolphin("notes", active_project.clone()),
            "11" => box_controls::stop_all_boxes(&projects),
            "12" => {project_controls::save_projects(&projects, &config_path);
                    let mut stop = String::new(); 
                    println!("stop all boxes?\ny/n");
                    std::io::stdin().read_line(&mut stop).unwrap();
                    if stop.contains("y"){
                        box_controls::stop_all_boxes(&projects);
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