use colored::Colorize;


pub fn print_project_not_found() {
    println!("mindless project not found.");
    println!("Use {} to start a new project.", "mindless init\n".blue().bold());
}

pub fn print_no_history() {
    println!("{}", "No history found in current workspace.".yellow().bold());
    println!("Use {} to make your first save or {} for help\n", "mindless save".blue().bold(), "mindless help".blue().bold());
}

pub fn print_project_created() {
    println!("{}", "New mindless project created".green().bold());
    println!("Use {} to save your project.\n", "mindless save".blue().bold());
}

pub fn print_project_already_exists() {
    println!("{}", "mindless project already exists.".red().bold());
    println!("Use {} to make your first save or {} for help\n", "mindless save".blue().bold(), "mindless help".blue().bold());
}
