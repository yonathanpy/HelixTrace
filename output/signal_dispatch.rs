pub struct SignalDispatch;

impl SignalDispatch {
    pub fn emit(msg: &str) {
        println!("[signal] {}", msg);
    }

    pub fn alert(level: u8, msg: &str) {
        println!("[alert:{}] {}", level, msg);
    }
}
