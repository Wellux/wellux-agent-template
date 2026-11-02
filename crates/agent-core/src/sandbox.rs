#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SandboxRuntime {
    /// Fast MicroVMs. Default for untrusted code.
    MicroVm,
    /// Full QEMU Linux, optional desktop, via the Quicksand bridge.
    Quicksand,
    /// Lightweight isolate when you do not need a guest OS.
    Wasm,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SandboxSpec {
    pub runtime: SandboxRuntime,
    pub image: String,
    pub memory_mb: u32,
}

pub struct SandboxManager;

impl SandboxManager {
    pub fn recommend(needs_desktop: bool, untrusted_code: bool) -> SandboxRuntime {
        if needs_desktop {
            SandboxRuntime::Quicksand
        } else if untrusted_code {
            SandboxRuntime::MicroVm
        } else {
            SandboxRuntime::Wasm
        }
    }

    pub fn spec(runtime: SandboxRuntime) -> SandboxSpec {
        let (image, memory_mb) = match runtime {
            SandboxRuntime::MicroVm => ("microsandbox:default", 512),
            SandboxRuntime::Quicksand => ("ubuntu", 2048),
            SandboxRuntime::Wasm => ("wasm:empty", 64),
        };
        SandboxSpec {
            runtime,
            image: image.to_string(),
            memory_mb,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn picks_the_smallest_runtime_that_fits() {
        assert_eq!(
            SandboxManager::recommend(true, true),
            SandboxRuntime::Quicksand
        );
        assert_eq!(
            SandboxManager::recommend(false, true),
            SandboxRuntime::MicroVm
        );
        assert_eq!(
            SandboxManager::recommend(false, false),
            SandboxRuntime::Wasm
        );
        assert_eq!(SandboxManager::spec(SandboxRuntime::Wasm).memory_mb, 64);
    }
}
