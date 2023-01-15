static terminal_output: &'static str = "
$ cd /
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

#[derive(Debug)]
struct File {
    path: &'static str,
    name: &'static str,
    size: u32,
}

struct Disk {
    files: Vec<File>,
    directories: Vec<&'static str>,
}

fn main() {
    resolve_directory(terminal_output);
}

fn resolve_directory(terminal_out: &'static str) -> Disk {
    let mut output = Disk { files: vec![], directories: vec![] };
    let mut current_path: Vec<&str> = vec![];

    fn process_command<'a>(path: &mut Vec<&'a str>, command: &'a str, outputs: &[&'static str]) -> () {
        let command: Vec<&str> = command.split(" ").collect();
        match command[0] {
            "cd" => {
                match command[1] {
                    "/" => {
                        path.clear();
                        path.push("/");
                    },
                    ".." => {
                        path.pop();
                    },
                    _ => {
                        path.push(command[1]);
                    }
                }
            },
            "ls" => {
                let path = path.join(" ");
                println!("outputs: {:?}", outputs);
                let mut files = vec![];
                for output in outputs {
                    let output: Vec<&str> = output.split(" ").collect();
                    files.push(File { path: path.as_str(), name: output[1], size: output[2].parse().unwrap()});
                    println!("{:?}", files);
                }
            },
            _ => panic!("command is not found")
        }
    }

    let terminal_out: Vec<&str> = terminal_out.split("\n$ ").collect();
    let terminal_out = &terminal_out[1..];

    for command in terminal_out {
        let command: Vec<&str> = command.split("\n").collect();
        let outputs = &command[1..];
        let command = &command[0];

        process_command(&mut current_path, command, outputs);
    }


    output
}
