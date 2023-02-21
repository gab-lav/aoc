use crate::utils;

struct Directory{
    name: String,
    files: Vec<u32>,
    subdirs: Vec<String>
}

pub fn day7(test: bool) {
    let input: String;
    if test {
        input = String::from("$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k");
    } else {
        input = utils::read_input(7);
    }

    let mut current_dir = String::from("/");
    let mut dirs: Vec<Directory> = Vec::new();

    for l in input.split("$").skip(2) {
        let cmd = &l[1..3];
        if cmd  == "ls" {
            dirs.push(decode_dir(&l[4..], &current_dir));
        } else {
            current_dir = change_dir(&current_dir, &l[4..]);
        } 
    }

    let mut part1: u32 = 0;

    let needed_space = 30000000 - (70000000 - get_directory_size("/", &dirs));
    let mut part2: u32 = 70000000;
    for d in &dirs {
        let name = &d.name;
        let size = get_directory_size(name, &dirs);
        if size <= 100000 {
            part1 += size;
        }
        if size >= needed_space && size <= part2 {
            part2 = size;
        }
    }
    println!("{part1}");
    println!("{part2}");
}

fn get_directory_size(name: &str, dirs: &Vec<Directory>) -> u32 {
    let mut size: u32 = 0;
    for d in dirs {
        if d.name == name {
            let files = &d.files;
            let subdirs = &d.subdirs;
            for f in files {
                size += f;
            }
            for s in subdirs {
                size += get_directory_size(&s, dirs)
            }
        }
    }
    return size
}

fn change_dir(current_dir: &str, cmd: &str) -> String {
    let mut new_dir = String::from(current_dir);
    if &cmd[..2] == ".." {
        new_dir = String::from("/");
        let mut parts: Vec<&str> = current_dir.split("/").collect();
        parts.retain(|x| x.len() != 0);
        if parts.len() != 0{
            for p in &parts[..parts.len()-1]{
                new_dir += p;
                new_dir += "/";
            }
        }
    } else {
        new_dir += cmd.trim();
        new_dir += "/";
    }
    return new_dir;
}

fn decode_dir(result: &str, name: &str) -> Directory {
    let mut dir = Directory {
        name: String::from(name), 
        files: Vec::new(),
        subdirs: Vec::new()
    };
    for l in result.split("\n") { 
        if l.len() == 0 {
            continue
        }
        let mut iter = l.split(" ");
        let first = iter.next().unwrap();
        let second = iter.next().unwrap();
        if first == "dir" {
            dir.subdirs.push(String::from(name) + second + "/");
        } else {
            dir.files.push(first.parse::<u32>().unwrap());
        }
    }
    return dir;
}
