use rscript::{
    scripting::{DynamicScript, FFiVec},
    Hook, ScriptInfo, ScriptType, VersionReq,
};
mod script;

#[no_mangle]
pub static SCRIPT: DynamicScript = DynamicScript {
    script,
    script_info,
};

static mut VIM: Vim = Vim::new();

extern "C" fn script_info() -> FFiVec {
    let info = ScriptInfo::new(
        "VimDylib",
        ScriptType::DynamicLib,
        &[
            irust_api::InputEvent::NAME,
            irust_api::Shutdown::NAME,
            irust_api::Startup::NAME,
        ],
        VersionReq::parse(">=1.30.2").expect("correct version requirement"),
    );
    FFiVec::serialize_from(&info).unwrap()
}

extern "C" fn script(name: FFiVec, data: FFiVec) -> FFiVec {
    let name: String = name.deserialize().unwrap();
    match name.as_str() {
        irust_api::InputEvent::NAME => {
            let hook: irust_api::InputEvent = data.deserialize().unwrap();
            let output: <irust_api::InputEvent as Hook>::Output =
                unsafe { VIM.handle_input_event(hook) };
            FFiVec::serialize_from(&output).unwrap()
        }
        irust_api::Shutdown::NAME => {
            let hook: irust_api::Shutdown = data.deserialize().unwrap();
            let output: <irust_api::Shutdown as Hook>::Output = unsafe { VIM.clean_up(hook) };
            FFiVec::serialize_from(&output).unwrap()
        }
        irust_api::Startup::NAME => {
            let hook: irust_api::Startup = data.deserialize().unwrap();
            let output: <irust_api::Startup as Hook>::Output = unsafe { VIM.start_up(hook) };
            FFiVec::serialize_from(&output).unwrap()
        }
        _ => unreachable!(),
    }
}

struct Vim {
    state: State,
    mode: Mode,
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
enum State {
    Empty,
    c,
    ci,
    d,
    di,
    g,
    f,
    F,
    r,
}

#[derive(PartialEq)]
enum Mode {
    Normal,
    Insert,
}