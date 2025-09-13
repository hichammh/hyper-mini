pub fn start_vm(id: String, mem_mb: u64) -> Result<String, String> {
    println!("(macOS) Pretending to start VM {id} with {mem_mb} MB...");
    Ok("mock-running".into())
}
