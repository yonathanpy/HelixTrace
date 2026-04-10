pub struct ErrorGuard;

impl ErrorGuard {
    pub fn wrap<F>(mut f: F)
    where
        F: FnMut() -> Result<(), String>,
    {
        match f() {
            Ok(_) => {}
            Err(e) => {
                eprintln!("[error] {}", e);
            }
        }
    }
}
