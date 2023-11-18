use crate::cmd::Extension;

pub fn function_dec(name: String, return_body: String, extension: Extension) -> String {
    match extension {
        Extension::JS => format!("export function is{name}(o){{return({return_body})}}"),
        Extension::TS => format!("export function is{name}(o: unknown): o is {name}{{return({return_body})}}"),
        Extension::DTS => panic!("File destination can't end with .d.ts"),
    }
}

pub fn return_body(entries_len: usize, return_body: String) -> String {
    format!("o!=null&&typeof o===\"object\"&&Object.keys(o).length==={entries_len}{return_body}")
}
