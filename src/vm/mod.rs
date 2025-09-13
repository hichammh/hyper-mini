#[cfg(target_os = "linux")]
pub mod kvm;
#[cfg(target_os = "macos")]
pub mod dummy;

pub fn start_vm(id: String, mem_mb: u64) -> Result<String, String> {
    #[cfg(target_os = "linux")]
    return kvm::start_vm(id, mem_mb);

    #[cfg(target_os = "macos")]
    return dummy::start_vm(id, mem_mb);
}
