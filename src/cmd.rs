#[derive(Copy, Clone)]
pub enum Extension {
    JS,
    TS,
    DTS,
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

pub const USAGE: &str = "USAGE: ts-runtime-tc-gen <READ-FILE-PATH> <WRITE-FILE-PATH>";