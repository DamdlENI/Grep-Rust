use std::fs;
use std::fs::File;
use std::io::Write;
use std::io::{self, BufRead};
use std::path::Path;

use regex::Regex;
use walkdir::WalkDir;

fn main() {
    
start();
}

fn start() {
println!("                                                                     ");
println!("                                                                     ");
println!("    ## ##   ### ##   ### ###    ##     ### ##   ### ###  ### ##      ");     
println!("   ##   ##   ##  ##   ##  ##     ##     ##  ##   ##  ##   ##  ##     ");     
println!("   ##        ##  ##   ##       ## ##    ##  ##   ##       ##  ##     ");     
println!("   ##  ###   ## ##    ## ##    ##  ##   ##  ##   ## ##    ## ##      ");     
println!("   ##   ##   ## ##    ##       ## ###   ## ##    ##       ## ##      ");     
println!("   ##   ##   ##  ##   ##  ##   ##  ##   ##       ##  ##   ##  ##     ");     
println!("    ## ##   #### ##  ### ###  ###  ##  ####     ### ###  #### ##     ");     
println!("                                                                     ");     
println!("   ### ##   ##  ##            ### ##     ##     ##   ##              ");
println!("    ##  ##  ##  ##             ##  ##     ##     ## ##               ");
println!("    ##  ##  ##  ##             ##  ##   ## ##   # ### #              ");
println!("    ## ##    ## ##             ##  ##   ##  ##  ## # ##              ");
println!("    ##  ##    ##               ##  ##   ## ###  ##   ##              ");
println!("    ##  ##    ##               ##  ##   ##  ##  ##   ##              ");
println!("   ### ##     ##              ### ##   ###  ##  ##   ##              ");
println!("                                                                     ");
println!("        [1] Start        [2] Help            [3] Infos               ");
println!("                                                                     ");
println!("                                                                     ");



choice()
}


fn choice() {


    let mut choice = String::new(); 

    println!(" ");
    
    io::stdin()
        .read_line(&mut choice)
        .expect("Échec de la lecture de l'entrée utilisateur");

        match choice.trim().parse::<i32>() {
            Ok(choice) => {
                if choice == 1 {
                    settings()
                }
                if choice == 2 {
                    help()
                }
                if choice == 3 {
                    infos()
                }
            } Err(_) => {
                println!("les soucis");
            }
        }
}




fn settings() {
    
    let chemin_set: String = (&"set.txt").to_string();

    let container = fs::read_to_string(&chemin_set);

    if container.expect("PROBLEME").is_empty() {
        setacces(chemin_set.clone())
    }
    else {
        search()
    }
}

fn setacces(chemin_set: String) {

    let chemin_set = ("set.txt").trim().to_string();
    
    let mut file = File::create(chemin_set.clone()).expect("error");

    let mut direction = String::new(); 

    println!("Precisez le chemin d'accès vers le dossier ou sont situé les DB ");
    
    io::stdin()
        .read_line(&mut direction)
        .expect("Error");

    let towrite = direction;
    file.write_all(towrite.as_bytes()).expect("Error");
    search()
}

fn search() {

        
    loop {
        
        let dir = read_directory_from_file().unwrap_or_else(|| ".".to_string());

        
        let mut input = String::new();

        println!("Entrez le pseudo à rechercher (ou tapez 'exit' pour quitter) : ");

        io::stdin()
        .read_line(&mut input)
        .expect("Error");

        let search_word = input.trim();

        
        if search_word.to_lowercase() == "exit" {
            break;
        }

        
        let file_extensions = vec!["txt", "sql"];

        
        let mut result_found = false;

        for entry in WalkDir::new(&dir).into_iter().filter_map(|e| e.ok()) {

            if entry.file_type().is_file() {
                
                if let Some(extension) = entry.path().extension().and_then(|e| e.to_str()) {

                    if file_extensions.contains(&extension) {
                       
                    if let Ok(file) = File::open(entry.path()) {

                    let reader = io::BufReader::new(file);

                    for (line_number, line) in reader.lines().enumerate() {
                    if let Ok(line_content) = line {
                                    
                    let re = Regex::new(search_word).expect("Error");
                        if re.is_match(&line_content) {
                        println!("            ");
                        println!("Ligne {} : {}", line_number + 1, line_content);
                        result_found = true;
                }
            }
        }
    }
    }
    }
}
  }

        
        if !result_found {
            println!("Aucun Résultat", search_word);
        }
    }
}

fn read_directory_from_file() -> Option<String> {
    if let Ok(file) = File::open("set.txt") {
        let reader = io::BufReader::new(file);
        if let Some(line) = reader.lines().next() {
            return Some(line.unwrap().trim().to_string());
        }
    }
    None
}

fn infos() {
    println!("By Dam | itisdam");
    choice()
}

fn help() {
    println!("Si vous rencontrez un problème avec la redirection videz tout le contenu de set.txt");
    choice()
}
