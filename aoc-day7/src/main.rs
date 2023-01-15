fn main() {
    let terminal_out = "$ cd /
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
7214296 k";

    resolve_directory(terminal_out);
    // let mut disk = resolve_directory(terminal_out);
    // println!("{:?}", disk);
}

struct File {
    name: &'static str,
    size: u32,
}

fn resolve_directory(terminal_out: &'static str)  {
    use std::collections::HashMap;
    let mut output = HashMap::new();

    let terminal_out: Vec<&str> = terminal_out.split("$ ").collect();
    let terminal_out = &terminal_out[1..];

    let mut current_path = vec!["/"];
    for command_out in terminal_out {
        let command_out: Vec<&str> = command_out.split("\n").collect();
        if command_out[0].starts_with("cd") {
            // cd
            let path: Vec<&str> = command_out[0].clone().split(" ").collect();
            let path = path[1];
            match path {
                "/" => {
                    current_path.clear();
                    current_path.push("/");
                },
                ".." => {
                    current_path.pop();
                }
                name => {
                    current_path.push(name);
                }
            }
        } else {
            // ls
            println!("ls ({:?})", current_path);
            let mut file_list = vec![];
            for file in &command_out[1..] {
                let file: Vec<&str> = file.split(" ").collect();

                match file[0] {
                    "dir" => (),
                    size => {
                        let size: u32 = size.parse().unwrap();

                        println!("parse: {}", size);
                        file_list.push(File { name: file[1], size: size });
                    },
                }
            }
            output.insert(current_path.join(" "), file_list);
        }
    }
}
