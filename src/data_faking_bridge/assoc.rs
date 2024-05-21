pub enum FNVARI {
    Bool(fn() -> bool),
    I8(fn() -> i8),
    String(fn() -> String)
}

pub fn get_func_from_string(arg: &String) -> FNVARI {
    match arg.as_str() {
        "Bool" => FNVARI::Bool(data_faking::data::defaults::types::bool),
        "I8" => FNVARI::I8(data_faking::data::defaults::types::i8),
        "UUID V4" => FNVARI::String(data_faking::data::defaults::uuids::uuid_v4_wasm),
        _ => panic!("{} - no function found for this string", arg)
    }
}
