# hyper-mini

**hyper-mini** is an experimental minimal hypervisor written in Rust.  
The goal is to provide a lightweight, memory-safe foundation for **micro-VMs** in cloud and edge environments.

> ⚠️ Status: Work in progress.  
> On macOS → only a dummy backend is available (for API development).  
> On Linux → KVM integration is being built for real VM execution.

---

## ✨ Features (MVP)

- REST API to start/stop/list VMs.
- CLI (via `curl` or client) to control the hypervisor.
- macOS dummy backend (no real VMs, for development only).
- Linux KVM backend (stubbed, ready for kernel + initramfs integration).
- Written in safe Rust with `tokio` + `axum`.

---

## 🖥️ Requirements

### macOS (development only)
- Rust (1.70+)
- macOS 13+ (Apple Silicon or Intel)

### Linux (real hypervisor mode)
- Linux kernel with `/dev/kvm` available.
- CPU with virtualization (Intel VT-x or AMD-V).
- Packages: `qemu-kvm`, `cpio`, `build-essential`.

Check KVM support:
```bash
egrep -c '(vmx|svm)' /proc/cpuinfo
ls /dev/kvm
