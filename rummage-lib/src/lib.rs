use llama_cpp::{LlamaModel, LlamaSession};
use rusqlite::Connection;
use std::os::raw::c_char;

pub struct Rummage {
    model: Option<LlamaModel>,
    context: Option<LlamaSession>,
    db: Option<Connection>,
    auth_token: Option<String>
}

impl Default for Rummage {
    fn default() -> Self {
        // TODO: make constructor
        // * (either log in to outlook OR pass access toke in)
        // * delete emails that are too old
        // * load llama
        // * initialize db
        println!("INIT RUMMAGE");
        return Self {model: None, context: None, db: None, auth_token: Option::Some("hello?".to_string())};
    }
}

impl Rummage {
    pub fn refresh(&self) {
        // TODO: pull emails, update last sync
        println!("REFRESH");
    }

    fn search_emails(&self, prompt: String) -> Vec<String> {
        // TODO: return relevant emails
        return vec!["id1".to_string(), "id2".to_string()];
    }
    
    pub fn prompt(&self, prompt: String) -> (String, Vec<String>) {
        // TODO: find relevant entries, prompt llm with prompt
        println!("prompt was: {}", prompt);
        let ids = self.search_emails(prompt);
        return ("Hello World".to_string(), ids);
    }

    pub fn kill_llama(&self) {
        // TODO: unload llama
        println!("KILL LLAMA");
    }

    pub fn generate_daily_update(&self) -> String {
        self.refresh();
        // TODO: 

        return "daily update".to_string();
    }
}

#[repr(C)] // Ensure the struct layout is compatible with C
pub struct ChatResponse {
    response: *mut c_char,
    email_ids: *mut *mut c_char,  // Array of strings (Vec<String>) converted to C-style pointers
    len: usize, // Number of strings in the Vec
}

pub mod c_api {
    use std::ffi::c_char;
    use std::ffi::{CStr, CString};

    use super::ChatResponse;

    use super::Rummage;

    #[no_mangle]
    pub extern "C" fn rummage_make() -> *mut std::ffi::c_void {
        let b = Box::new(Rummage::default());
        return Box::into_raw(b).cast();
    }

    #[no_mangle]
    pub extern "C" fn rummage_refresh(ptr: *mut std::ffi::c_void) {
        let b = unsafe { Box::from_raw(ptr.cast::<Rummage>()) };
        b.refresh();
        std::mem::forget(b);
    }

    #[no_mangle]
    pub extern "C" fn rummage_prompt(ptr: *mut std::ffi::c_void, prompt: *mut c_char) -> ChatResponse {
        let b = unsafe { Box::from_raw(ptr.cast::<Rummage>()) };
        let prompt_str = unsafe { CStr::from_ptr(prompt) }.to_str().unwrap().to_string();

        let res = b.prompt(prompt_str);
        std::mem::forget(b);
        let len = res.1.len();

        let email_ids: Vec<*mut c_char> = res.1
            .into_iter()
            .map(|s| CString::new(s).unwrap().into_raw())
            .collect();

        let email_ptr = Box::into_raw(email_ids.into_boxed_slice()) as *mut *mut c_char;

        ChatResponse {
            response: CString::new(res.0).unwrap().into_raw(),
            email_ids: email_ptr,
            len: len
        }
    }

    #[no_mangle]
    pub extern "C" fn rummage_kill_llama(ptr: *mut std::ffi::c_void) {
        let b = unsafe { Box::from_raw(ptr.cast::<Rummage>()) };
        b.kill_llama();
        std::mem::forget(b);
    }

    #[no_mangle]
    pub extern "C" fn rummage_generate_daily_update(ptr: *mut std::ffi::c_void) -> *mut c_char {
        let b = unsafe { Box::from_raw(ptr.cast::<Rummage>()) };
        let res = b.generate_daily_update();
        std::mem::forget(b);

        return CString::new(res).unwrap().into_raw();
    }

    #[no_mangle]
    pub extern "C" fn rummage_free(ptr: *mut std::ffi::c_void) {
        let b = unsafe { Box::from_raw(ptr) };
        drop(b)
    }
}