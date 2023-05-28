mod file;

use file::File;

const INPUT: &str = include_str!("input.txt");

fn input_to_system<'a>(input: &'a str) -> File<'a> {
    let mut dirs: Vec<File> = Vec::new();

    let commands: Vec<_> = input
        .split("$ ")
        .skip(1)
        .map(|cmd_out| cmd_out.lines().collect::<Vec<&str>>())
        .collect();

    for command in commands.iter() {
        if command[0].starts_with("ls") {
            for line in command
                .iter()
                .skip(1)
                .filter(|line| !line.starts_with("dir"))
            {
                let tmp: Vec<&str> = line.split(" ").collect();
                let last_index = dirs.len() - 1;
                dirs[last_index].insert(File::new_file(tmp[1], tmp[0].parse().unwrap()));
            }
        } else if command[0].starts_with("cd") {
            let tmp: Vec<&str> = command[0].split(" ").collect();
            if tmp[1] != ".." {
                dirs.push(File::new_dir(tmp[1]));
            } else {
                let dir = dirs.pop().unwrap();
                let last_index = dirs.len() - 1;
                dirs[last_index].insert(dir);
            }
        }
    }

    // if we didn't cd .. from last dir we entered
    while dirs.len() != 1 {
        let dir = dirs.pop().unwrap();
        let last_index = dirs.len() - 1;
        dirs[last_index].insert(dir);
    }

    dirs.pop().unwrap()
}

fn get_total(root: &File) -> u32 {
    if root.is_file() {
        0
    } else {
        if !root.has_subdir() {
            let mut val = root.get_size();
            val = if val <= 100_000 { val } else { 0 };
            return val;
        }

        let mut sum: u32 = 0;
        for file in root.open_dir() {
            if file.is_dir() {
                sum += get_total(file);
            }
        }

        let mut val = root.get_size();
        val = if val <= 100_000 { val } else { 0 };

        sum + val
    }
}

fn get_all_dir(root: &File, store: &mut Vec<u32>) {
    if root.is_dir() {
        if !root.has_subdir() {
            store.push(root.get_size());
            return;
        }

        for file in root.open_dir() {
            if file.is_dir() {
                get_all_dir(file, store);
            }
        }

        store.push(root.get_size());
    }
}

fn delete_dir(root: &File) -> Option<u32> {
    let required = root.get_size() as i32 - 40_000_000;
    let mut tmp: Vec<_> = Vec::new();
    get_all_dir(root, &mut tmp);
    tmp.sort();

    for size in tmp {
        if (size as i32) >= required {
            return Some(size);
        }
    }
    None
}

fn main() {
    let root = input_to_system(INPUT);
    println!("{}", root);
    println!("Total size: {}", get_total(&root));
    println!("Clean size: {}", delete_dir(&root).unwrap());
}
