pub fn function_dec(name: String, return_body: String) -> String {
    format!("export function is{name}(o: unknown): o is {name}{{return({return_body})}}")
}

pub fn return_body(entries_len: usize, return_body: String) -> String {
    format!("o!=null&&typeof o===\"object\"&&Object.keys(o).length==={entries_len}{return_body}")
}
