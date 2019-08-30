#[cfg(target_arch = "x86")]
const ARCH: &str = "x86";

#[cfg(target_arch = "x86_64")]
const ARCH: &str = "x64";

#[cfg(target_arch = "mips")]
const ARCH: &str = "mips";

#[cfg(target_arch = "powerpc")]
const ARCH: &str = "powerpc";

#[cfg(target_arch = "powerpc64")]
const ARCH: &str = "powerpc64";

#[cfg(target_arch = "arm")]
const ARCH: &str = "ARM";

#[cfg(target_arch = "aarch64")]
const ARCH: &str = "ARM64";

fn main() {
    println!("Hello, world!");
    println!("Compiled for {}", ARCH);
}
