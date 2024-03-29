use crate::tcb::misc::netlist_unmodified;
#[cfg(feature = "verify")]
use crate::tcb::path::path_safe;
use crate::tcb::sbox_mem::{raw_ptr, valid_linmem};
use crate::tcb::verifier::trace::{Effect, EffectType, Trace};
use crate::types::{addr_in_netlist, VmCtx, HOMEDIR_FD, LINEAR_MEM_SIZE};
use prusti_contracts::*;

#[cfg(feature = "verify")]
predicate! {
    pub fn ctx_safe(ctx: &VmCtx) -> bool {
        //let mem_ptr = raw_ptr(ctx.mem.as_slice());
        ctx.memlen == LINEAR_MEM_SIZE &&
        ctx.argc < 1024 &&
        ctx.envc < 1024 &&
        ctx.arg_buffer.len() < 1024 * 1024 &&
        ctx.env_buffer.len() < 1024 * 1024 &&
        netlist_unmodified(&ctx.netlist) &&
        valid_linmem(raw_ptr(ctx.mem.as_slice())) //&&
        //mem_ptr <= mem_ptr + count
    }
}

#[cfg(feature = "verify")]
predicate! {
    pub fn trace_safe(trace: &Trace, ctx: &VmCtx) -> bool {
        forall(|i: usize|
            (i < trace.len() ==> (
                match trace.lookup(i) {
                    Effect { typ: EffectType::ReadMem | EffectType::WriteMem, f1: addr, f2: count, .. } => {
                        let mem_ptr = raw_ptr(ctx.mem.as_slice());
                        valid_linmem(mem_ptr) && // sbox mem has been initialized
                        addr >= mem_ptr && // start of buffer in sbox mem
                        addr + count < mem_ptr + ctx.memlen && // end of buffer in sbox mem
                        mem_ptr <= mem_ptr + ctx.memlen && // memory region does not overflow address space
                        addr <= addr + count // buffer does not overflow
                    },
                    //Effect { typ: EffectType::WriteMem, f1: addr, f2: count, .. } => valid_linmem(raw_ptr(ctx.mem.as_slice())),//(addr < ctx.memlen) && (count < ctx.memlen) && (addr <= (addr + count)),
                    Effect { typ: EffectType::Shutdown, ..  } => true, // currently, all shutdowns are safe
                    Effect { typ: EffectType::FdAccess, ..  } => true,
                    Effect { typ: EffectType::PathAccessAt, f1: dir_fd, f2:_, f3:_, p: Some(path), should_follow: Some(b) } => dir_fd == ctx.homedir_host_fd.to_raw() && path.len() == 4096 && path_safe(&path, b),
                    Effect { typ: EffectType::NetAccess, f1: _proto, f2:addr, f3:port, .. } => addr_in_netlist(&ctx.netlist, addr as u32, port as u32),
                    Effect { typ: EffectType::SockCreation, f1: domain, f2:ty, ..  } => domain == (libc::AF_INET as usize) && (ty == (libc::SOCK_STREAM as usize) || ty == (libc::SOCK_DGRAM as usize)),
                    _ => false,
                }
            ))
        )
    }
}

// pub fn path_safe(v: &HostPath, should_follow: bool) -> bool {
//     arr_is_relative(&v) &&
//     (arr_depth(&v) >= 0) &&
//     (should_follow ==> !arr_is_symlink(&v)) &&
//     arr_has_no_symlink_prefixes(&v)
// }


// #[ensures(effects!(old(trace), trace, effect!(PathAccessAt, dirfd, path, !flag_set(flags, libc::O_NOFOLLOW))
// pub fn os_openat(dirfd: usize, path: [u8; PATH_MAX], flags: i32) -> isize {
