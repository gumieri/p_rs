use git2::Repository;
use semver::Version;
use std::env;
use std::fs::metadata;
use std::path::{Path, PathBuf};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        print_projetcs();
        return;
    }

    if args[1] == "tag" {
        tag();
        return;
    }
}

fn tag() {
    let path = match env::current_dir() {
        Ok(path) => path,
        Err(e) => panic!("failed to open: {}", e),
    };

    let repo = match Repository::open(path) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };

    let tags = match repo.tag_names(None) {
        Ok(tags) => tags,
        Err(e) => panic!("failed to open: {}", e),
    };

    for tag in tags.iter() {
        if let Some(tag_name) = tag {
            println!("{}", tag_name);
            Version::parse(tag_name);
        }
    }
}

fn print_projetcs() {
    let projects_path = match get_projects_path() {
        Ok(path) => path,
        Err(_e) => {
            eprintln!("Failed to identify the Projects Path.");
            return;
        }
    };

    loop_path(&projects_path, &projects_path);
}

fn get_projects_path() -> Result<PathBuf, env::VarError> {
    match env::var("PROJECTS_PATH") {
        Ok(env_config) => Ok(Path::new(&env_config).to_path_buf()),
        Err(_e) => Ok(Path::new(&env::var("HOME")?).join(Path::new("Projects"))),
    }
}

fn loop_path(prefix: &Path, node: &Path) {
    let md = metadata(&node).unwrap();
    if !md.is_dir() {
        return;
    }

    let paths = std::fs::read_dir(&node).unwrap();

    let mut found = false;
    for path in paths {
        if path.unwrap().path().ends_with(".git") {
            found = true;
            break;
        }
    }

    if found {
        let project = node.strip_prefix(prefix).unwrap().to_str().unwrap();
        println!("{}", project);
        return;
    }

    let paths = std::fs::read_dir(&node).unwrap();
    for path in paths {
        let path = path.unwrap().path();
        loop_path(prefix, path.as_path());
    }
}
