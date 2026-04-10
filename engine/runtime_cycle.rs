use crate::control::error_guard::ErrorGuard;

pub struct RuntimeCycle;

impl RuntimeCycle {
    pub fn execute<F>(mut f: F)
    where
        F: FnMut() -> Result<(), String>,
    {
        ErrorGuard::wrap(|| {
            f()?;
            Ok(())
        });
    }
}
