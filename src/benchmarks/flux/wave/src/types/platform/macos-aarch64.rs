use super::super::*;
use crate::tcb::misc::*;
use crate::types::*;
use libc;

/// On Mac, posix_advise doesn't exist. Just have the call do nothing.
impl From<Advice> for i32 {
    fn from(advice: Advice) -> Self {
        0
    }
}

impl TryFrom<i32> for Advice {
    type Error = RuntimeError;
    fn try_from(advice: i32) -> RuntimeResult<Self> {
        Ok(Advice::Normal)
    }
}

impl FdFlags {
    pub fn to_posix(&self) -> i32 {
        let mut flags = 0;
        // TODO: DSYNC, RSYNC, and SYNC do not exist on mac. Ignoring for now...
        if nth_bit_set(self.0, 0) {
            flags = bitwise_or(flags, libc::O_APPEND)
        }
        if nth_bit_set(self.0, 2) {
            flags = bitwise_or(flags, libc::O_NONBLOCK)
        }
        flags
    }

    pub fn from_posix(flags: i32) -> Self {
        // FdFlags(flags as u16)
        //let mut result = FdFlags(0);
        // TODO: DSYNC, RSYNC, and SYNC do not exist on mac. Ignoring for now...
        let mut result = FdFlags(0);
        if bitwise_and(flags, libc::O_APPEND) != 0 {
            result.0 = with_nth_bit_set(result.0, 0);
        }
        if bitwise_and(flags, libc::O_NONBLOCK) != 0 {
            result.0 = with_nth_bit_set(result.0, 2);
        }
        result
    }
}

impl Dirent {
    // #[requires(in_idx < host_buf.len())]
    // #[flux::sig(fn (host_buf: &RVec<u8>[@len], in_idx: usize{in_idx + 7 < len}) -> Result<Dirent, RuntimeError>)]
    pub fn parse(host_buf: &RVec<u8>, in_idx: usize) -> Result<Dirent, RuntimeError> {
        if in_idx + 7 >= host_buf.len() {
            return Err(RuntimeError::Eoverflow);
        }
        // Inode number
        let d_ino = u32::from_le_bytes([
            host_buf[in_idx + 0],
            host_buf[in_idx + 1],
            host_buf[in_idx + 2],
            host_buf[in_idx + 3],
        ]);

        // Offset to next linux_dirent
        let d_reclen = u16::from_le_bytes([host_buf[in_idx + 4], host_buf[in_idx + 5]]);

        // File type
        let d_type = host_buf[in_idx + 6];

        // Length of this linux_dirent
        let d_namlen = host_buf[in_idx + 7];

        // If we would overflow - don't :)
        if d_reclen < 8 || (in_idx + d_reclen as usize) > host_buf.len() {
            return Err(RuntimeError::Eoverflow);
        }

        let out_namlen = first_null(&host_buf, in_idx, 8, d_reclen as usize);
        let dirent = Dirent {
            ino: d_ino as u64,
            reclen: d_reclen,
            name_start: 8,
            out_namlen,
            typ: d_type,
        };

        Ok(dirent)
    }
}

impl SockAddr {
    #[flux::trusted]
    #[flux::sig(fn(sin_family: u8, sin_port: u16, sin_addr: u32) -> SockAddr[sin_port, sin_addr])]
    pub fn new(sin_family: u8, sin_port: u16, sin_addr: u32) -> Self {
        SockAddr {
            inner: libc::sockaddr_in {
                // i'll be lazy, should refactor to os-specific code...
                #[cfg(target_os = "macos")]
                sin_len: 0,
                sin_family,
                sin_port,
                sin_addr: libc::in_addr { s_addr: sin_addr },
                sin_zero: [0; 8],
            },
        }
    }
}
