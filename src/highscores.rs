use std::fs::File;
use std::fs::OpenOptions;
use std::fs::remove_file;
use std::io::Write;
use std::io::Read;

pub fn read_highscores() -> String {
    let mut file;
    let mut contents = String::new();

    if let Ok(f) = File::open("resources/highscores.txt") {
        file = f;
        file.read_to_string(&mut contents)
            .expect("Can't read the file!");
    } else {
       file = File::create("resources/highscores.txt").unwrap();
       file.write_all("0\n".as_bytes()).expect("cant write to file");
    }

    contents
}

pub fn add_highscore(score: i32) -> bool
{
    let mut file = OpenOptions::new().read(true).write(true).open("resources/highscores.txt").expect("no highscore file found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
    .expect("Can't read the file!");

    let end = contents.find('\n');
        if end.is_some()
        {
            let string = &contents[0..end.unwrap()];
            if score > string.parse::<i32>().unwrap() {
                let newscore = &score.to_string();
                let result =  &contents.replace(string, &newscore);

                remove_file("resources/highscores.txt").expect("Can not remove file");
                file = File::create("resources/highscores.txt").unwrap();
                file.write_all(result.as_bytes()).expect("cant write to file");         
                return true
            }
        }
        return false
    }
