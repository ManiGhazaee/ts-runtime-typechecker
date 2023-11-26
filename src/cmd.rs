use std::env;

#[derive(Copy, Clone)]
pub enum Extension {
    JS,
    TS,
    DTS,
}

pub fn input() -> (String, String, Extension) {
    let args: Vec<String> = env::args().collect();
    if let (Some(f), Some(w)) = (args.get(1), args.get(2)) {
        (f.clone(), w.clone(), get_extension(w.clone()))
    } else {
        eprintln!("{}", USAGE);
        panic!();
    }
}

pub fn get_extension(file_path: String) -> Extension {
    if file_path.ends_with(".js") {
        return Extension::JS;
    } else if file_path.ends_with(".d.ts") {
        return Extension::DTS;
    } else if file_path.ends_with(".ts") {
        return Extension::TS;
    } else {
        panic!("File extension is not .ts or .js");
    }
}

pub const USAGE: &str = "USAGE: ts-runtime-typechecker <READ-FILE-PATH> <WRITE-FILE-PATH>";
