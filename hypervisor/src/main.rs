//#![no_std]
#![feature(allocator_api)]
#![feature(new_uninit)]

use error::HypervisorError;
use vmx::VMX;

use crate::processor::{processor_count, ProcessorExecutor};

mod processor;
mod alloc;
mod nt;
mod vmx;
mod error;

fn main() {
    match init_vmx() {
        Ok(_) => log::info!("[+] VMM initialized"),
        Err(err) => log::error!("[-] VMM initialization failed: {}", err),
    }
}

fn init_vmx() -> Result<(), HypervisorError> {
    //
    // 1) Intel Manual: 24.6 Discover Support for Virtual Machine Extension (VMX)
    //

    let vmx = VMX::new();

    vmx.has_intel_cpu()?;
    log::info!("[+] CPU is Intel");

    vmx.has_vmx_support()?;
    log::info!("[+] Virtual Machine Extension (VMX) technology is supported");

    //
    // 2) Intel Manual: 24.7 Enable and Enter VMX Operation
    //

    for i in 0..processor_count() {
        
        ProcessorExecutor::switch_to_processor(i);

        vmx.enable_vmx_operation()?;
        log::info!("[+] Virtual Machine Extensions (VMX) enabled");
    
        vmx.adjust_control_registers();
        log::info!("[+] Control registers adjusted");


        let vmxon_pa = vmx.allocate_vmm_context()?;
        vmx.vmxon(vmxon_pa)?;
        log::info!("[+] VMXON successful!");

        let vmptrld_pa = vmx.allocate_vmm_context()?;
        vmx.vmptrld(vmptrld_pa)?;
        log::info!("[+] VMPTRLD successful!");
    }

    return Ok(());
}

//pub fn init_logical_processor() {}
