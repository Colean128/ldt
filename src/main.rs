use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
       println!("List Directory in Terminal (ldt) by Colean128. Input a directory.")
    } else {
        let dir = &args[1];
        println!("{}", dir);
        match fs:: read_dir(dir) {
           Err(why) => eprintln!("! {:?}", why.kind()),
           Ok(paths) => for path in paths {
               let path_raw = path.unwrap().path();
               let path_lossy_string = path_raw.to_string_lossy();
               let path_len = path_lossy_string.len();
               let path_dir_len = dir.len() + 1;
               let path_cut_len = path_len - path_dir_len;
               let final_path_string: String = path_lossy_string.chars().rev().take(path_cut_len).collect();
               let bugger: String = final_path_string.chars().rev().collect();
                
               println!("> {}", bugger);
            }
        }
   }
}
