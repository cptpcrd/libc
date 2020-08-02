// APIs that changed in FreeBSD12

pub type nlink_t = u64;
pub type dev_t = u64;
pub type ino_t = ::c_ulong;
pub type shmatt_t = ::c_uint;

s! {
    pub struct shmid_ds {
        pub shm_perm: ::ipc_perm,
        pub shm_segsz: ::size_t,
        pub shm_lpid: ::pid_t,
        pub shm_cpid: ::pid_t,
        pub shm_nattch: ::shmatt_t,
        pub shm_atime: ::time_t,
        pub shm_dtime: ::time_t,
        pub shm_ctime: ::time_t,
    }

    pub struct kevent {
        pub ident: ::uintptr_t,
        pub filter: ::c_short,
        pub flags: ::c_ushort,
        pub fflags: ::c_uint,
        pub data: ::intptr_t,
        pub udata: *mut ::c_void,
        pub ext: [u64; 4],
    }
}

s_no_extra_traits! {
    pub struct dirent {
        pub d_fileno: ::ino_t,
        pub d_off: ::off_t,
        pub d_reclen: u16,
        pub d_type: u8,
        d_pad0: u8,
        pub d_namlen: u16,
        d_pad1: u16,
        pub d_name: [::c_char; 256],
    }

    pub struct statfs {
        pub f_version: u32,
        pub f_type: u32,
        pub f_flags: u64,
        pub f_bsize: u64,
        pub f_iosize: u64,
        pub f_blocks: u64,
        pub f_bfree: u64,
        pub f_bavail: i64,
        pub f_files: u64,
        pub f_ffree: i64,
        pub f_syncwrites: u64,
        pub f_asyncwrites: u64,
        pub f_syncreads: u64,
        pub f_asyncreads: u64,
        f_spare: [u64; 10],
        pub f_namemax: u32,
        pub f_owner: ::uid_t,
        pub f_fsid: ::fsid_t,
        f_charspare: [::c_char; 80],
        pub f_fstypename: [::c_char; 16],
        pub f_mntfromname: [::c_char; 1024],
        pub f_mntonname: [::c_char; 1024],
    }

    pub struct priority {
        pub pri_class: u8,
        pub pri_level: u8,
        pub pri_native: u8,
        pub pri_user: u8,
    }

    pub struct kinfo_proc {
        pub ki_structsize: ::c_int,
        pub ki_layout: ::c_int,
        pub ki_args: *mut ::c_void,
        pub ki_paddr: *mut ::c_void,
        pub ki_addr: *mut ::c_void,
        pub ki_tracep: *mut ::c_void,
        pub ki_textvp: *mut ::c_void,
        pub ki_fd: *mut ::c_void,
        pub ki_vmspace: *mut ::c_void,
        pub ki_wchan: *const ::c_void,
        pub ki_pid: ::pid_t,
        pub ki_ppid: ::pid_t,
        pub ki_pgid: ::pid_t,
        pub ki_tpgid: ::pid_t,
        pub ki_sid: ::pid_t,
        pub ki_tsid: ::pid_t,
        pub ki_jobc: ::c_short,
        ki_spare_short1: ::c_short,
        pub ki_tdev_freebsd11: u32,
        pub ki_siglist: ::sigset_t,
        pub ki_sigmask: ::sigset_t,
        pub ki_sigignore: ::sigset_t,
        pub ki_sigcatch: ::sigset_t,
        pub ki_uid: ::uid_t,
        pub ki_ruid: ::uid_t,
        pub ki_svuid: ::uid_t,
        pub ki_rgid: ::gid_t,
        pub ki_svgid: ::gid_t,
        pub ki_ngroups: ::c_short,
        ki_spare_short2: ::c_short,
        pub ki_groups: [::gid_t; ::KI_NGROUPS as usize],
        pub ki_size: ::vm_size_t,
        pub ki_rssize: ::segsz_t,
        pub ki_swrss: ::segsz_t,
        pub ki_tsize: ::segsz_t,
        pub ki_dsize: ::segsz_t,
        pub ki_ssize: ::segsz_t,
        pub ki_xstat: ::c_ushort,
        pub ki_acflag: ::c_ushort,
        pub ki_pctcpu: ::fixpt_t,
        pub ki_estcpu: ::c_uint,
        pub ki_slptime: ::c_uint,
        pub ki_swtime: ::c_uint,
        pub ki_cow: ::c_uint,
        pub ki_runtime: i64,
        pub ki_start: ::timeval,
        pub ki_childtime: ::timeval,
        pub ki_flag: ::c_long,
        pub ki_kiflag: ::c_long,
        pub ki_traceflag: ::c_int,
        pub ki_stat: ::c_char,
        pub ki_nice: i8,
        pub ki_lock: ::c_char,
        pub ki_rqindex: ::c_char,
        pub ki_oncpu_old: u8,
        pub ki_lastcpu_old: u8,
        pub ki_tdname: [::c_char; ::TDNAMLEN as usize + 1],
        pub ki_wmesg: [::c_char; ::WMESGLEN as usize + 1],
        pub ki_login: [::c_char; ::LOGNAMELEN as usize + 1],
        pub ki_lockname: [::c_char; ::LOCKNAMELEN as usize + 1],
        pub ki_comm: [::c_char; ::COMMLEN as usize + 1],
        pub ki_emul: [::c_char; ::KI_EMULNAMELEN as usize + 1],
        pub ki_loginclass: [::c_char; ::LOGINCLASSLEN as usize + 1],
        pub ki_moretdname: [::c_char; ::MAXCOMLEN as usize - ::TDNAMLEN as usize + 1],
        ki_sparestrings: [::c_char; 46],
        ki_spareints: [::c_int; 2],
        pub ki_tdev: ::dev_t,
        pub ki_oncpu: ::c_int,
        pub ki_lastcpu: ::c_int,
        pub ki_tracer: ::c_int,
        pub ki_flag2: ::c_int,
        pub ki_fibnum: ::c_int,
        pub ki_cr_flags: ::c_uint,
        pub ki_jid: ::c_int,
        pub ki_numthreads: ::c_int,
        pub ki_tid: ::lwpid_t,
        pub ki_pri: ::priority,
        pub ki_rusage: ::rusage,
        pub ki_rusage_ch: ::rusage,
        pub ki_pcb: *mut ::c_void,
        pub ki_kstack: *mut ::c_void,
        pub ki_udata: *mut ::c_void,
        pub ki_tdaddr: *mut ::c_void,
        ki_spareptrs: [*mut ::c_void; 6],
        ki_sparelongs: [::c_long; 12],
        pub ki_sflag: ::c_long,
        pub ki_tdflags: ::c_long,
    }
}

cfg_if! {
    if #[cfg(feature = "extra_traits")] {
        impl PartialEq for statfs {
            fn eq(&self, other: &statfs) -> bool {
                self.f_version == other.f_version
                    && self.f_type == other.f_type
                    && self.f_flags == other.f_flags
                    && self.f_bsize == other.f_bsize
                    && self.f_iosize == other.f_iosize
                    && self.f_blocks == other.f_blocks
                    && self.f_bfree == other.f_bfree
                    && self.f_bavail == other.f_bavail
                    && self.f_files == other.f_files
                    && self.f_ffree == other.f_ffree
                    && self.f_syncwrites == other.f_syncwrites
                    && self.f_asyncwrites == other.f_asyncwrites
                    && self.f_syncreads == other.f_syncreads
                    && self.f_asyncreads == other.f_asyncreads
                    && self.f_namemax == other.f_namemax
                    && self.f_owner == other.f_owner
                    && self.f_fsid == other.f_fsid
                    && self.f_fstypename == other.f_fstypename
                    && self
                    .f_mntfromname
                    .iter()
                    .zip(other.f_mntfromname.iter())
                    .all(|(a,b)| a == b)
                    && self
                    .f_mntonname
                    .iter()
                    .zip(other.f_mntonname.iter())
                    .all(|(a,b)| a == b)
            }
        }
        impl Eq for statfs {}
        impl ::fmt::Debug for statfs {
            fn fmt(&self, f: &mut ::fmt::Formatter) -> ::fmt::Result {
                f.debug_struct("statfs")
                    .field("f_bsize", &self.f_bsize)
                    .field("f_iosize", &self.f_iosize)
                    .field("f_blocks", &self.f_blocks)
                    .field("f_bfree", &self.f_bfree)
                    .field("f_bavail", &self.f_bavail)
                    .field("f_files", &self.f_files)
                    .field("f_ffree", &self.f_ffree)
                    .field("f_syncwrites", &self.f_syncwrites)
                    .field("f_asyncwrites", &self.f_asyncwrites)
                    .field("f_syncreads", &self.f_syncreads)
                    .field("f_asyncreads", &self.f_asyncreads)
                    .field("f_namemax", &self.f_namemax)
                    .field("f_owner", &self.f_owner)
                    .field("f_fsid", &self.f_fsid)
                    .field("f_fstypename", &self.f_fstypename)
                    .field("f_mntfromname", &&self.f_mntfromname[..])
                    .field("f_mntonname", &&self.f_mntonname[..])
                    .finish()
            }
        }
        impl ::hash::Hash for statfs {
            fn hash<H: ::hash::Hasher>(&self, state: &mut H) {
                self.f_version.hash(state);
                self.f_type.hash(state);
                self.f_flags.hash(state);
                self.f_bsize.hash(state);
                self.f_iosize.hash(state);
                self.f_blocks.hash(state);
                self.f_bfree.hash(state);
                self.f_bavail.hash(state);
                self.f_files.hash(state);
                self.f_ffree.hash(state);
                self.f_syncwrites.hash(state);
                self.f_asyncwrites.hash(state);
                self.f_syncreads.hash(state);
                self.f_asyncreads.hash(state);
                self.f_namemax.hash(state);
                self.f_owner.hash(state);
                self.f_fsid.hash(state);
                self.f_charspare.hash(state);
                self.f_fstypename.hash(state);
                self.f_mntfromname.hash(state);
                self.f_mntonname.hash(state);
            }
        }

        impl PartialEq for dirent {
            fn eq(&self, other: &dirent) -> bool {
                self.d_fileno == other.d_fileno
                    && self.d_off == other.d_off
                    && self.d_reclen == other.d_reclen
                    && self.d_type == other.d_type
                    && self.d_namlen == other.d_namlen
                    && self
                    .d_name[..self.d_namlen as _]
                    .iter()
                    .zip(other.d_name.iter())
                    .all(|(a,b)| a == b)
            }
        }
        impl Eq for dirent {}
        impl ::fmt::Debug for dirent {
            fn fmt(&self, f: &mut ::fmt::Formatter) -> ::fmt::Result {
                f.debug_struct("dirent")
                    .field("d_fileno", &self.d_fileno)
                    .field("d_off", &self.d_off)
                    .field("d_reclen", &self.d_reclen)
                    .field("d_type", &self.d_type)
                    .field("d_namlen", &self.d_namlen)
                    .field("d_name", &&self.d_name[..self.d_namlen as _])
                    .finish()
            }
        }
        impl ::hash::Hash for dirent {
            fn hash<H: ::hash::Hasher>(&self, state: &mut H) {
                self.d_fileno.hash(state);
                self.d_off.hash(state);
                self.d_reclen.hash(state);
                self.d_type.hash(state);
                self.d_namlen.hash(state);
                self.d_name[..self.d_namlen as _].hash(state);
            }
        }
    }
}

pub const F_ADD_SEALS: ::c_int = 19;
pub const F_GET_SEALS: ::c_int = 20;
pub const F_SEAL_SEAL: ::c_int = 0x0001;
pub const F_SEAL_SHRINK: ::c_int = 0x0002;
pub const F_SEAL_GROW: ::c_int = 0x0004;
pub const F_SEAL_WRITE: ::c_int = 0x0008;

pub const GRND_NONBLOCK: ::c_uint = 0x1;
pub const GRND_RANDOM: ::c_uint = 0x2;

cfg_if! {
    if #[cfg(not(freebsd13))] {
        pub const ELAST: ::c_int = 96;
    } else {
        pub const EINTEGRITY: ::c_int = 97;
        pub const ELAST: ::c_int = 97;
        pub const GRND_INSECURE: ::c_uint = 0x4;
    }
}

extern "C" {
    pub fn setgrent();
    pub fn mprotect(
        addr: *mut ::c_void,
        len: ::size_t,
        prot: ::c_int,
    ) -> ::c_int;
    pub fn freelocale(loc: ::locale_t);
    pub fn msgrcv(
        msqid: ::c_int,
        msgp: *mut ::c_void,
        msgsz: ::size_t,
        msgtyp: ::c_long,
        msgflg: ::c_int,
    ) -> ::ssize_t;
    pub fn clock_nanosleep(
        clk_id: ::clockid_t,
        flags: ::c_int,
        rqtp: *const ::timespec,
        rmtp: *mut ::timespec,
    ) -> ::c_int;

    pub fn fdatasync(fd: ::c_int) -> ::c_int;

    pub fn getrandom(
        buf: *mut ::c_void,
        buflen: ::size_t,
        flags: ::c_uint
    ) -> ::ssize_t;
}

cfg_if! {
    if #[cfg(any(target_arch = "x86_64",
                 target_arch = "aarch64"))] {
        mod b64;
        pub use self::b64::*;
    }
}
