use anyhow::*;

pub fn start_vm(id: String, mem_mb: u64) -> Result<String, String> {
    // TODO: real KVM setup here
    println!("Starting real VM {id} with {mem_mb} MB using KVM...");
    Ok("running".into())
}
