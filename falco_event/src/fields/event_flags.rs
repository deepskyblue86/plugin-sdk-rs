use falco_event_derive::event_flags;

event_flags! {
type file_flags: PT_FLAGS32;
type clone_flags: PT_FLAGS32;
type prot_flags: PT_FLAGS32;
type mmap_flags: PT_FLAGS32;
type splice_flags: PT_FLAGS32;
type quotactl_cmds: PT_FLAGS16;
type quotactl_types: PT_FLAGS8;
type quotactl_dqi_flags: PT_FLAGS8;
type quotactl_quota_fmts: PT_FLAGS8;
type flock_flags: PT_FLAGS32;
type semop_flags: PT_FLAGS16;
type mount_flags: PT_FLAGS32;
type umount_flags: PT_FLAGS32;
type semget_flags: PT_FLAGS32;
type pf_flags: PT_FLAGS32;
type execve_flags: PT_FLAGS32;
type unlinkat_flags: PT_FLAGS32;
type linkat_flags: PT_FLAGS32;
type renameat2_flags: PT_FLAGS32;
type openat2_flags: PT_FLAGS32;
type execveat_flags: PT_FLAGS32;
type mlockall_flags: PT_FLAGS32;
type semctl_commands: PT_FLAGS16;
type access_flags: PT_FLAGS32;
type io_uring_setup_flags: PT_FLAGS32;
type io_uring_setup_feats: PT_FLAGS32;
type io_uring_enter_flags: PT_FLAGS32;
type mlock2_flags: PT_FLAGS32;
type epoll_create1_flags: PT_FLAGS32;
type file_flags: PT_FLAGS16;
type memfd_create_flags: PT_FLAGS32;
type newfstatat_flags: PT_FLAGS32;
type delete_module_flags: PT_FLAGS32;
type fchownat_flags: PT_FLAGS32;
type mknod_mode: PT_MODE;
type chmod_mode: PT_MODE;
type socket_families: PT_ENUMFLAGS32 !PPM_AF_LOCAL, PPM_AF_ROUTE;
type shutdown_how: PT_ENUMFLAGS8;
type sockopt_levels: PT_ENUMFLAGS8;
type sockopt_options: PT_ENUMFLAGS8;
type futex_operations: PT_ENUMFLAGS16;
type lseek_whence: PT_ENUMFLAGS8;
type rlimit_resources: PT_ENUMFLAGS8;
type fcntl_commands: PT_ENUMFLAGS8;
type ptrace_requests: PT_ENUMFLAGS16;
type io_uring_register_opcodes: PT_ENUMFLAGS16;
type bpf_commands: PT_ENUMFLAGS32 !PPM_BPF_PROG_TEST_RUN;
type fsconfig_cmds: PT_ENUMFLAGS32;
type prctl_options: PT_ENUMFLAGS32;
type pidfd_open_flags: PT_FLAGS32;
type finit_module_flags: PT_FLAGS32;
const struct ppm_name_value socket_families[] = {
    {"AF_NFC", PPM_AF_NFC},
    {"AF_ALG", PPM_AF_ALG},
    {"AF_CAIF", PPM_AF_CAIF},
    {"AF_IEEE802154", PPM_AF_IEEE802154},
    {"AF_PHONET", PPM_AF_PHONET},
    {"AF_ISDN", PPM_AF_ISDN},
    {"AF_RXRPC", PPM_AF_RXRPC},
    {"AF_IUCV", PPM_AF_IUCV},
    {"AF_BLUETOOTH", PPM_AF_BLUETOOTH},
    {"AF_TIPC", PPM_AF_TIPC},
    {"AF_CAN", PPM_AF_CAN},
    {"AF_LLC", PPM_AF_LLC},
    {"AF_WANPIPE", PPM_AF_WANPIPE},
    {"AF_PPPOX", PPM_AF_PPPOX},
    {"AF_IRDA", PPM_AF_IRDA},
    {"AF_SNA", PPM_AF_SNA},
    {"AF_RDS", PPM_AF_RDS},
    {"AF_ATMSVC", PPM_AF_ATMSVC},
    {"AF_ECONET", PPM_AF_ECONET},
    {"AF_ASH", PPM_AF_ASH},
    {"AF_PACKET", PPM_AF_PACKET},
    {"AF_ROUTE", PPM_AF_ROUTE},
    {"AF_NETLINK", PPM_AF_NETLINK},
    {"AF_KEY", PPM_AF_KEY},
    {"AF_SECURITY", PPM_AF_SECURITY},
    {"AF_NETBEUI", PPM_AF_NETBEUI},
    {"AF_DECnet", PPM_AF_DECnet},
    {"AF_ROSE", PPM_AF_ROSE},
    {"AF_INET6", PPM_AF_INET6},
    {"AF_X25", PPM_AF_X25},
    {"AF_ATMPVC", PPM_AF_ATMPVC},
    {"AF_BRIDGE", PPM_AF_BRIDGE},
    {"AF_NETROM", PPM_AF_NETROM},
    {"AF_APPLETALK", PPM_AF_APPLETALK},
    {"AF_IPX", PPM_AF_IPX},
    {"AF_AX25", PPM_AF_AX25},
    {"AF_INET", PPM_AF_INET},
    {"AF_LOCAL", PPM_AF_LOCAL},
    {"AF_UNIX", PPM_AF_UNIX},
    {"AF_UNSPEC", PPM_AF_UNSPEC},
    {0, 0},
};

const struct ppm_name_value file_flags[] = {
    {"O_LARGEFILE", PPM_O_LARGEFILE},
    {"O_DIRECTORY", PPM_O_DIRECTORY},
    {"O_DIRECT", PPM_O_DIRECT},
    {"O_TRUNC", PPM_O_TRUNC},
    {"O_SYNC", PPM_O_SYNC},
    {"O_NONBLOCK", PPM_O_NONBLOCK},
    {"O_EXCL", PPM_O_EXCL},
    {"O_DSYNC", PPM_O_DSYNC},
    {"O_APPEND", PPM_O_APPEND},
    {"O_CREAT", PPM_O_CREAT},
    {"O_RDWR", PPM_O_RDWR},
    {"O_WRONLY", PPM_O_WRONLY},
    {"O_RDONLY", PPM_O_RDONLY},
    {"O_CLOEXEC", PPM_O_CLOEXEC},
    {"O_NONE", PPM_O_NONE},
    {"O_TMPFILE", PPM_O_TMPFILE},
    {"O_F_CREATED", PPM_O_F_CREATED},
    {0, 0},
};

const struct ppm_name_value flock_flags[] = {
    {"LOCK_SH", PPM_LOCK_SH},
    {"LOCK_EX", PPM_LOCK_EX},
    {"LOCK_NB", PPM_LOCK_NB},
    {"LOCK_UN", PPM_LOCK_UN},
    {"LOCK_NONE", PPM_LOCK_NONE},
    {0, 0},
};

const struct ppm_name_value clone_flags[] = {
    {"CLONE_FILES", PPM_CL_CLONE_FILES},
    {"CLONE_FS", PPM_CL_CLONE_FS},
    {"CLONE_IO", PPM_CL_CLONE_IO},
    {"CLONE_NEWIPC", PPM_CL_CLONE_NEWIPC},
    {"CLONE_NEWNET", PPM_CL_CLONE_NEWNET},
    {"CLONE_NEWNS", PPM_CL_CLONE_NEWNS},
    {"CLONE_NEWPID", PPM_CL_CLONE_NEWPID},
    {"CLONE_NEWUTS", PPM_CL_CLONE_NEWUTS},
    {"CLONE_PARENT", PPM_CL_CLONE_PARENT},
    {"CLONE_PARENT_SETTID", PPM_CL_CLONE_PARENT_SETTID},
    {"CLONE_PTRACE", PPM_CL_CLONE_PTRACE},
    {"CLONE_SIGHAND", PPM_CL_CLONE_SIGHAND},
    {"CLONE_SYSVSEM", PPM_CL_CLONE_SYSVSEM},
    {"CLONE_THREAD", PPM_CL_CLONE_THREAD},
    {"CLONE_UNTRACED", PPM_CL_CLONE_UNTRACED},
    {"CLONE_VM", PPM_CL_CLONE_VM},
    {"CLONE_INVERTED", PPM_CL_CLONE_INVERTED},
    {"NAME_CHANGED", PPM_CL_NAME_CHANGED},
    {"CLOSED", PPM_CL_CLOSED},
    {"CLONE_NEWUSER", PPM_CL_CLONE_NEWUSER},
    {"CLONE_CHILD_CLEARTID", PPM_CL_CLONE_CHILD_CLEARTID},
    {"CLONE_CHILD_SETTID", PPM_CL_CLONE_CHILD_SETTID},
    {"CLONE_SETTLS", PPM_CL_CLONE_SETTLS},
    {"CLONE_STOPPED", PPM_CL_CLONE_STOPPED},
    {"CLONE_VFORK", PPM_CL_CLONE_VFORK},
    {"CLONE_NEWCGROUP", PPM_CL_CLONE_NEWCGROUP},
    {0, 0},
};

const struct ppm_name_value futex_operations[] = {
    {"FUTEX_CLOCK_REALTIME", PPM_FU_FUTEX_CLOCK_REALTIME},
    {"FUTEX_PRIVATE_FLAG", PPM_FU_FUTEX_PRIVATE_FLAG},
    {"FUTEX_CMP_REQUEUE_PI", PPM_FU_FUTEX_CMP_REQUEUE_PI},
    {"FUTEX_WAIT_REQUEUE_PI", PPM_FU_FUTEX_WAIT_REQUEUE_PI},
    {"FUTEX_WAKE_BITSET", PPM_FU_FUTEX_WAKE_BITSET},
    {"FUTEX_WAIT_BITSET", PPM_FU_FUTEX_WAIT_BITSET},
    {"FUTEX_TRYLOCK_PI", PPM_FU_FUTEX_TRYLOCK_PI},
    {"FUTEX_UNLOCK_PI", PPM_FU_FUTEX_UNLOCK_PI},
    {"FUTEX_LOCK_PI", PPM_FU_FUTEX_LOCK_PI},
    {"FUTEX_WAKE_OP", PPM_FU_FUTEX_WAKE_OP},
    {"FUTEX_CMP_REQUEUE", PPM_FU_FUTEX_CMP_REQUEUE},
    {"FUTEX_REQUEUE", PPM_FU_FUTEX_REQUEUE},
    {"FUTEX_FD", PPM_FU_FUTEX_FD},
    {"FUTEX_WAKE", PPM_FU_FUTEX_WAKE},
    {"FUTEX_WAIT", PPM_FU_FUTEX_WAIT},
    {0, 0},
};

const struct ppm_name_value poll_flags[] = {
    {"POLLIN", PPM_POLLIN},
    {"POLLPRI", PPM_POLLPRI},
    {"POLLOUT", PPM_POLLOUT},
    {"POLLRDHUP", PPM_POLLRDHUP},
    {"POLLERR", PPM_POLLERR},
    {"POLLHUP", PPM_POLLHUP},
    {"POLLNVAL", PPM_POLLNVAL},
    {"POLLRDNORM", PPM_POLLRDNORM},
    {"POLLRDBAND", PPM_POLLRDBAND},
    {"POLLWRNORM", PPM_POLLWRNORM},
    {"POLLWRBAND", PPM_POLLWRBAND},
    {0, 0},
};

/* http://lxr.free-electrons.com/source/include/uapi/linux/fs.h?v=4.2#L65 */
const struct ppm_name_value mount_flags[] = {
    {"RDONLY", PPM_MS_RDONLY},
    {"NOSUID", PPM_MS_NOSUID},
    {"NODEV", PPM_MS_NODEV},
    {"NOEXEC", PPM_MS_NOEXEC},
    {"SYNCHRONOUS", PPM_MS_SYNCHRONOUS},
    {"REMOUNT", PPM_MS_REMOUNT},
    {"MANDLOCK", PPM_MS_MANDLOCK},
    {"DIRSYNC", PPM_MS_DIRSYNC},
    {"NOATIME", PPM_MS_NOATIME},
    {"NODIRATIME", PPM_MS_NODIRATIME},
    {"BIND", PPM_MS_BIND},
    {"MOVE", PPM_MS_MOVE},
    {"REC", PPM_MS_REC},
    {"SILENT", PPM_MS_SILENT},
    {"POSIXACL", PPM_MS_POSIXACL},
    {"UNBINDABLE", PPM_MS_UNBINDABLE},
    {"PRIVATE", PPM_MS_PRIVATE},
    {"SLAVE", PPM_MS_SLAVE},
    {"SHARED", PPM_MS_SHARED},
    {"RELATIME", PPM_MS_RELATIME},
    {"KERNMOUNT", PPM_MS_KERNMOUNT},
    {"I_VERSION", PPM_MS_I_VERSION},
    {"STRICTATIME", PPM_MS_STRICTATIME},
    {"LAZYTIME", PPM_MS_LAZYTIME},
    {"NOSEC", PPM_MS_NOSEC},
    {"BORN", PPM_MS_BORN},
    {"ACTIVE", PPM_MS_ACTIVE},
    {"NOUSER", PPM_MS_NOUSER}, // NOTE: we are at 1 << 31 -> and we have an uint32_t value.
    {0, 0},
};

/* There is a 1:1 mapping between `umount2` flags and our `PPM` notation, so we don't
 * need a dedicated helper for the conversion.
 */
const struct ppm_name_value umount_flags[] = {
    {"FORCE", PPM_MNT_FORCE},
    {"DETACH", PPM_MNT_DETACH},
    {"EXPIRE", PPM_MNT_EXPIRE},
    {"NOFOLLOW", PPM_UMOUNT_NOFOLLOW},
    {0, 0},
};

const struct ppm_name_value lseek_whence[] = {
    {"SEEK_END", PPM_SEEK_END},
    {"SEEK_CUR", PPM_SEEK_CUR},
    {"SEEK_SET", PPM_SEEK_SET},
    {0, 0},
};

const struct ppm_name_value shutdown_how[] = {
    {"SHUT_RDWR", PPM_SHUT_RDWR},
    {"SHUT_WR", PPM_SHUT_WR},
    {"SHUT_RD", PPM_SHUT_RD},
    {0, 0},
};

const struct ppm_name_value rlimit_resources[] = {
    {"RLIMIT_UNKNOWN", PPM_RLIMIT_UNKNOWN},
    {"RLIMIT_RTTIME", PPM_RLIMIT_RTTIME},
    {"RLIMIT_RTPRIO", PPM_RLIMIT_RTPRIO},
    {"RLIMIT_NICE", PPM_RLIMIT_NICE},
    {"RLIMIT_MSGQUEUE", PPM_RLIMIT_MSGQUEUE},
    {"RLIMIT_SIGPENDING", PPM_RLIMIT_SIGPENDING},
    {"RLIMIT_LOCKS", PPM_RLIMIT_LOCKS},
    {"RLIMIT_AS", PPM_RLIMIT_AS},
    {"RLIMIT_MEMLOCK", PPM_RLIMIT_MEMLOCK},
    {"RLIMIT_NOFILE", PPM_RLIMIT_NOFILE},
    {"RLIMIT_NPROC", PPM_RLIMIT_NPROC},
    {"RLIMIT_RSS", PPM_RLIMIT_RSS},
    {"RLIMIT_CORE", PPM_RLIMIT_CORE},
    {"RLIMIT_STACK", PPM_RLIMIT_STACK},
    {"RLIMIT_DATA", PPM_RLIMIT_DATA},
    {"RLIMIT_FSIZE", PPM_RLIMIT_FSIZE},
    {"RLIMIT_CPU", PPM_RLIMIT_CPU},
    {0, 0},
};

const struct ppm_name_value fcntl_commands[] = {
    {"F_GETPIPE_SZ", PPM_FCNTL_F_GETPIPE_SZ},
    {"F_SETPIPE_SZ", PPM_FCNTL_F_SETPIPE_SZ},
    {"F_NOTIFY", PPM_FCNTL_F_NOTIFY},
    {"F_DUPFD_CLOEXEC", PPM_FCNTL_F_DUPFD_CLOEXEC},
    {"F_CANCELLK", PPM_FCNTL_F_CANCELLK},
    {"F_GETLEASE", PPM_FCNTL_F_GETLEASE},
    {"F_SETLEASE", PPM_FCNTL_F_SETLEASE},
    {"F_GETOWN_EX", PPM_FCNTL_F_GETOWN_EX},
    {"F_SETOWN_EX", PPM_FCNTL_F_SETOWN_EX},
    {"F_SETLKW64", PPM_FCNTL_F_SETLKW64},
    {"F_SETLK64", PPM_FCNTL_F_SETLK64},
    {"F_GETLK64", PPM_FCNTL_F_GETLK64},
    {"F_GETSIG", PPM_FCNTL_F_GETSIG},
    {"F_SETSIG", PPM_FCNTL_F_SETSIG},
    {"F_GETOWN", PPM_FCNTL_F_GETOWN},
    {"F_SETOWN", PPM_FCNTL_F_SETOWN},
    {"F_SETLKW", PPM_FCNTL_F_SETLKW},
    {"F_SETLK", PPM_FCNTL_F_SETLK},
    {"F_GETLK", PPM_FCNTL_F_GETLK},
    {"F_SETFL", PPM_FCNTL_F_SETFL},
    {"F_GETFL", PPM_FCNTL_F_GETFL},
    {"F_SETFD", PPM_FCNTL_F_SETFD},
    {"F_GETFD", PPM_FCNTL_F_GETFD},
    {"F_DUPFD", PPM_FCNTL_F_DUPFD},
    {"F_OFD_GETLK", PPM_FCNTL_F_OFD_GETLK},
    {"F_OFD_SETLK", PPM_FCNTL_F_OFD_SETLK},
    {"F_OFD_SETLKW", PPM_FCNTL_F_OFD_SETLKW},
    {"UNKNOWN", PPM_FCNTL_UNKNOWN},
    {0, 0},
};

const struct ppm_name_value sockopt_levels[] = {
    {"SOL_SOCKET", PPM_SOCKOPT_LEVEL_SOL_SOCKET},
    {"SOL_TCP", PPM_SOCKOPT_LEVEL_SOL_TCP},
    {"UNKNOWN", PPM_SOCKOPT_LEVEL_UNKNOWN},
    {0, 0},
};

const struct ppm_name_value sockopt_options[] = {
    {"SO_COOKIE", PPM_SOCKOPT_SO_COOKIE},
    {"SO_MEMINFO", PPM_SOCKOPT_SO_MEMINFO},
    {"SO_PEERGROUPS", PPM_SOCKOPT_SO_PEERGROUPS},
    {"SO_ATTACH_BPF", PPM_SOCKOPT_SO_ATTACH_BPF},
    {"SO_INCOMING_CPU", PPM_SOCKOPT_SO_INCOMING_CPU},
    {"SO_BPF_EXTENSIONS", PPM_SOCKOPT_SO_BPF_EXTENSIONS},
    {"SO_MAX_PACING_RATE", PPM_SOCKOPT_SO_MAX_PACING_RATE},
    {"SO_BUSY_POLL", PPM_SOCKOPT_SO_BUSY_POLL},
    {"SO_SELECT_ERR_QUEUE", PPM_SOCKOPT_SO_SELECT_ERR_QUEUE},
    {"SO_LOCK_FILTER", PPM_SOCKOPT_SO_LOCK_FILTER},
    {"SO_NOFCS", PPM_SOCKOPT_SO_NOFCS},
    {"SO_PEEK_OFF", PPM_SOCKOPT_SO_PEEK_OFF},
    {"SO_WIFI_STATUS", PPM_SOCKOPT_SO_WIFI_STATUS},
    {"SO_RXQ_OVFL", PPM_SOCKOPT_SO_RXQ_OVFL},
    {"SO_DOMAIN", PPM_SOCKOPT_SO_DOMAIN},
    {"SO_PROTOCOL", PPM_SOCKOPT_SO_PROTOCOL},
    {"SO_TIMESTAMPING", PPM_SOCKOPT_SO_TIMESTAMPING},
    {"SO_MARK", PPM_SOCKOPT_SO_MARK},
    {"SO_TIMESTAMPNS", PPM_SOCKOPT_SO_TIMESTAMPNS},
    {"SO_PASSSEC", PPM_SOCKOPT_SO_PASSSEC},
    {"SO_PEERSEC", PPM_SOCKOPT_SO_PEERSEC},
    {"SO_ACCEPTCONN", PPM_SOCKOPT_SO_ACCEPTCONN},
    {"SO_TIMESTAMP", PPM_SOCKOPT_SO_TIMESTAMP},
    {"SO_PEERNAME", PPM_SOCKOPT_SO_PEERNAME},
    {"SO_DETACH_FILTER", PPM_SOCKOPT_SO_DETACH_FILTER},
    {"SO_ATTACH_FILTER", PPM_SOCKOPT_SO_ATTACH_FILTER},
    {"SO_BINDTODEVICE", PPM_SOCKOPT_SO_BINDTODEVICE},
    {"SO_SECURITY_ENCRYPTION_NETWORK", PPM_SOCKOPT_SO_SECURITY_ENCRYPTION_NETWORK},
    {"SO_SECURITY_ENCRYPTION_TRANSPORT", PPM_SOCKOPT_SO_SECURITY_ENCRYPTION_TRANSPORT},
    {"SO_SECURITY_AUTHENTICATION", PPM_SOCKOPT_SO_SECURITY_AUTHENTICATION},
    {"SO_SNDTIMEO", PPM_SOCKOPT_SO_SNDTIMEO},
    {"SO_RCVTIMEO", PPM_SOCKOPT_SO_RCVTIMEO},
    {"SO_SNDLOWAT", PPM_SOCKOPT_SO_SNDLOWAT},
    {"SO_RCVLOWAT", PPM_SOCKOPT_SO_RCVLOWAT},
    {"SO_PEERCRED", PPM_SOCKOPT_SO_PEERCRED},
    {"SO_PASSCRED", PPM_SOCKOPT_SO_PASSCRED},
    {"SO_REUSEPORT", PPM_SOCKOPT_SO_REUSEPORT},
    {"SO_BSDCOMPAT", PPM_SOCKOPT_SO_BSDCOMPAT},
    {"SO_LINGER", PPM_SOCKOPT_SO_LINGER},
    {"SO_PRIORITY", PPM_SOCKOPT_SO_PRIORITY},
    {"SO_NO_CHECK", PPM_SOCKOPT_SO_NO_CHECK},
    {"SO_OOBINLINE", PPM_SOCKOPT_SO_OOBINLINE},
    {"SO_KEEPALIVE", PPM_SOCKOPT_SO_KEEPALIVE},
    {"SO_RCVBUFFORCE", PPM_SOCKOPT_SO_RCVBUFFORCE},
    {"SO_SNDBUFFORCE", PPM_SOCKOPT_SO_SNDBUFFORCE},
    {"SO_RCVBUF", PPM_SOCKOPT_SO_RCVBUF},
    {"SO_SNDBUF", PPM_SOCKOPT_SO_SNDBUF},
    {"SO_BROADCAST", PPM_SOCKOPT_SO_BROADCAST},
    {"SO_DONTROUTE", PPM_SOCKOPT_SO_DONTROUTE},
    {"SO_ERROR", PPM_SOCKOPT_SO_ERROR},
    {"SO_TYPE", PPM_SOCKOPT_SO_TYPE},
    {"SO_REUSEADDR", PPM_SOCKOPT_SO_REUSEADDR},
    {"SO_DEBUG", PPM_SOCKOPT_SO_DEBUG},
    {"UNKNOWN", PPM_SOCKOPT_UNKNOWN},
    {0, 0},
};

const struct ppm_name_value ptrace_requests[] = {
    {"PTRACE_SINGLEBLOCK", PPM_PTRACE_SINGLEBLOCK},
    {"PTRACE_SYSEMU_SINGLESTEP", PPM_PTRACE_SYSEMU_SINGLESTEP},
    {"PTRACE_SYSEMU", PPM_PTRACE_SYSEMU},
    {"PTRACE_ARCH_PRCTL", PPM_PTRACE_ARCH_PRCTL},
    {"PTRACE_SET_THREAD_AREA", PPM_PTRACE_SET_THREAD_AREA},
    {"PTRACE_GET_THREAD_AREA", PPM_PTRACE_GET_THREAD_AREA},
    {"PTRACE_OLDSETOPTIONS", PPM_PTRACE_OLDSETOPTIONS},
    {"PTRACE_SETFPXREGS", PPM_PTRACE_SETFPXREGS},
    {"PTRACE_GETFPXREGS", PPM_PTRACE_GETFPXREGS},
    {"PTRACE_SETFPREGS", PPM_PTRACE_SETFPREGS},
    {"PTRACE_GETFPREGS", PPM_PTRACE_GETFPREGS},
    {"PTRACE_SETREGS", PPM_PTRACE_SETREGS},
    {"PTRACE_GETREGS", PPM_PTRACE_GETREGS},
    {"PTRACE_SETSIGMASK", PPM_PTRACE_SETSIGMASK},
    {"PTRACE_GETSIGMASK", PPM_PTRACE_GETSIGMASK},
    {"PTRACE_PEEKSIGINFO", PPM_PTRACE_PEEKSIGINFO},
    {"PTRACE_LISTEN", PPM_PTRACE_LISTEN},
    {"PTRACE_INTERRUPT", PPM_PTRACE_INTERRUPT},
    {"PTRACE_SEIZE", PPM_PTRACE_SEIZE},
    {"PTRACE_SETREGSET", PPM_PTRACE_SETREGSET},
    {"PTRACE_GETREGSET", PPM_PTRACE_GETREGSET},
    {"PTRACE_SETSIGINFO", PPM_PTRACE_SETSIGINFO},
    {"PTRACE_GETSIGINFO", PPM_PTRACE_GETSIGINFO},
    {"PTRACE_GETEVENTMSG", PPM_PTRACE_GETEVENTMSG},
    {"PTRACE_SETOPTIONS", PPM_PTRACE_SETOPTIONS},
    {"PTRACE_SYSCALL", PPM_PTRACE_SYSCALL},
    {"PTRACE_DETACH", PPM_PTRACE_DETACH},
    {"PTRACE_ATTACH", PPM_PTRACE_ATTACH},
    {"PTRACE_SINGLESTEP", PPM_PTRACE_SINGLESTEP},
    {"PTRACE_KILL", PPM_PTRACE_KILL},
    {"PTRACE_CONT", PPM_PTRACE_CONT},
    {"PTRACE_POKEUSR", PPM_PTRACE_POKEUSR},
    {"PTRACE_POKEDATA", PPM_PTRACE_POKEDATA},
    {"PTRACE_POKETEXT", PPM_PTRACE_POKETEXT},
    {"PTRACE_PEEKUSR", PPM_PTRACE_PEEKUSR},
    {"PTRACE_PEEKDATA", PPM_PTRACE_PEEKDATA},
    {"PTRACE_PEEKTEXT", PPM_PTRACE_PEEKTEXT},
    {"PTRACE_TRACEME", PPM_PTRACE_TRACEME},
    {"PTRACE_UNKNOWN", PPM_PTRACE_UNKNOWN},
    {0, 0},
};

const struct ppm_name_value prot_flags[] = {
    {"PROT_READ", PPM_PROT_READ},
    {"PROT_WRITE", PPM_PROT_WRITE},
    {"PROT_EXEC", PPM_PROT_EXEC},
    {"PROT_SEM", PPM_PROT_SEM},
    {"PROT_GROWSDOWN", PPM_PROT_GROWSDOWN},
    {"PROT_GROWSUP", PPM_PROT_GROWSUP},
    {"PROT_SAO", PPM_PROT_SAO},
    {"PROT_NONE", PPM_PROT_NONE},
    {0, 0},
};

const struct ppm_name_value mmap_flags[] = {
    {"MAP_SHARED", PPM_MAP_SHARED},
    {"MAP_PRIVATE", PPM_MAP_PRIVATE},
    {"MAP_FIXED", PPM_MAP_FIXED},
    {"MAP_ANONYMOUS", PPM_MAP_ANONYMOUS},
    {"MAP_32BIT", PPM_MAP_32BIT},
    {"MAP_RENAME", PPM_MAP_RENAME},
    {"MAP_NORESERVE", PPM_MAP_NORESERVE},
    {"MAP_POPULATE", PPM_MAP_POPULATE},
    {"MAP_NONBLOCK", PPM_MAP_NONBLOCK},
    {"MAP_GROWSDOWN", PPM_MAP_GROWSDOWN},
    {"MAP_DENYWRITE", PPM_MAP_DENYWRITE},
    {"MAP_EXECUTABLE", PPM_MAP_EXECUTABLE},
    {"MAP_INHERIT", PPM_MAP_INHERIT},
    {"MAP_FILE", PPM_MAP_FILE},
    {"MAP_LOCKED", PPM_MAP_LOCKED},
    {0, 0},
};

const struct ppm_name_value splice_flags[] = {
    {"SPLICE_F_MOVE", PPM_SPLICE_F_MOVE},
    {"SPLICE_F_NONBLOCK", PPM_SPLICE_F_NONBLOCK},
    {"SPLICE_F_MORE", PPM_SPLICE_F_MORE},
    {"SPLICE_F_GIFT", PPM_SPLICE_F_GIFT},
    {0, 0},
};

const struct ppm_name_value quotactl_dqi_flags[] = {
    {"DQF_NONE", PPM_DQF_NONE},
    {"V1_DQF_RSQUASH", PPM_V1_DQF_RSQUASH},
    {0, 0},
};

const struct ppm_name_value quotactl_cmds[] = {
    {"Q_QUOTAON", PPM_Q_QUOTAON},
    {"Q_QUOTAOFF", PPM_Q_QUOTAOFF},
    {"Q_GETFMT", PPM_Q_GETFMT},
    {"Q_GETINFO", PPM_Q_GETINFO},
    {"Q_SETINFO", PPM_Q_SETINFO},
    {"Q_GETQUOTA", PPM_Q_GETQUOTA},
    {"Q_SETQUOTA", PPM_Q_SETQUOTA},
    {"Q_SYNC", PPM_Q_SYNC},
    {"Q_XQUOTAON", PPM_Q_XQUOTAON},
    {"Q_XQUOTAOFF", PPM_Q_XQUOTAOFF},
    {"Q_XGETQUOTA", PPM_Q_XGETQUOTA},
    {"Q_XSETQLIM", PPM_Q_XSETQLIM},
    {"Q_XGETQSTAT", PPM_Q_XGETQSTAT},
    {"Q_XQUOTARM", PPM_Q_XQUOTARM},
    {"Q_XQUOTASYNC", PPM_Q_XQUOTASYNC},
    {0, 0},
};

const struct ppm_name_value quotactl_types[] = {
    {"USRQUOTA", PPM_USRQUOTA},
    {"GRPQUOTA", PPM_GRPQUOTA},
    {0, 0},
};

const struct ppm_name_value quotactl_quota_fmts[] = {
    {"QFMT_NOT_USED", PPM_QFMT_NOT_USED},
    {"QFMT_VFS_OLD", PPM_QFMT_VFS_OLD},
    {"QFMT_VFS_V0", PPM_QFMT_VFS_V0},
    {"QFMT_VFS_V1", PPM_QFMT_VFS_V1},
    {0, 0},
};

const struct ppm_name_value semop_flags[] = {
    {"IPC_NOWAIT", PPM_IPC_NOWAIT},
    {"SEM_UNDO", PPM_SEM_UNDO},
    {0, 0},
};

const struct ppm_name_value semget_flags[] = {
    {"IPC_EXCL", PPM_IPC_EXCL},
    {"IPC_CREAT", PPM_IPC_CREAT},
    {0, 0},
};

const struct ppm_name_value semctl_commands[] = {
    {"IPC_STAT", PPM_IPC_STAT},
    {"IPC_SET", PPM_IPC_SET},
    {"IPC_RMID", PPM_IPC_RMID},
    {"IPC_INFO", PPM_IPC_INFO},
    {"SEM_INFO", PPM_SEM_INFO},
    {"SEM_STAT", PPM_SEM_STAT},
    {"GETALL", PPM_GETALL},
    {"GETNCNT", PPM_GETNCNT},
    {"GETPID", PPM_GETPID},
    {"GETVAL", PPM_GETVAL},
    {"GETZCNT", PPM_GETZCNT},
    {"SETALL", PPM_SETALL},
    {"SETVAL", PPM_SETVAL},
    {0, 0},
};

const struct ppm_name_value access_flags[] = {
    {"F_OK", PPM_F_OK},
    {"R_OK", PPM_R_OK},
    {"W_OK", PPM_W_OK},
    {"X_OK", PPM_X_OK},
    {0, 0},
};

const struct ppm_name_value pf_flags[] = {
    {"PROTECTION_VIOLATION", PPM_PF_PROTECTION_VIOLATION},
    {"PAGE_NOT_PRESENT", PPM_PF_PAGE_NOT_PRESENT},
    {"WRITE_ACCESS", PPM_PF_WRITE_ACCESS},
    {"READ_ACCESS", PPM_PF_READ_ACCESS},
    {"USER_FAULT", PPM_PF_USER_FAULT},
    {"SUPERVISOR_FAULT", PPM_PF_SUPERVISOR_FAULT},
    {"RESERVED_PAGE", PPM_PF_RESERVED_PAGE},
    {"INSTRUCTION_FETCH", PPM_PF_INSTRUCTION_FETCH},
    {0, 0},
};

const struct ppm_name_value unlinkat_flags[] = {
    {"AT_REMOVEDIR", PPM_AT_REMOVEDIR},
    {0, 0},
};

const struct ppm_name_value linkat_flags[] = {
    {"AT_SYMLINK_FOLLOW", PPM_AT_SYMLINK_FOLLOW},
    {"AT_EMPTY_PATH", PPM_AT_EMPTY_PATH},
    {0, 0},
};

const struct ppm_name_value newfstatat_flags[] = {
    {"AT_EMPTY_PATH", PPM_AT_EMPTY_PATH},
    {"AT_NO_AUTOMOUNT", PPM_AT_NO_AUTOMOUNT},
    {"AT_SYMLINK_NOFOLLOW", PPM_AT_SYMLINK_NOFOLLOW},
    {0, 0},
};

const struct ppm_name_value chmod_mode[] = {
    {"S_IXOTH", PPM_S_IXOTH},
    {"S_IWOTH", PPM_S_IWOTH},
    {"S_IROTH", PPM_S_IROTH},
    {"S_IXGRP", PPM_S_IXGRP},
    {"S_IWGRP", PPM_S_IWGRP},
    {"S_IRGRP", PPM_S_IRGRP},
    {"S_IXUSR", PPM_S_IXUSR},
    {"S_IWUSR", PPM_S_IWUSR},
    {"S_IRUSR", PPM_S_IRUSR},
    {"S_ISVTX", PPM_S_ISVTX},
    {"S_ISGID", PPM_S_ISGID},
    {"S_ISUID", PPM_S_ISUID},
    {0, 0},
};

const struct ppm_name_value fchownat_flags[] = {
    {"AT_SYMLINK_NOFOLLOW", PPM_AT_SYMLINK_FOLLOW},
    {"AT_EMPTY_PATH", PPM_AT_EMPTY_PATH},
    {0, 0},
};

const struct ppm_name_value renameat2_flags[] = {
    {"RENAME_NOREPLACE", PPM_RENAME_NOREPLACE},
    {"RENAME_EXCHANGE", PPM_RENAME_EXCHANGE},
    {"RENAME_WHITEOUT", PPM_RENAME_WHITEOUT},
    {0, 0},
};

const struct ppm_name_value openat2_flags[] = {
    {"RESOLVE_BENEATH", PPM_RESOLVE_BENEATH},
    {"RESOLVE_IN_ROOT", PPM_RESOLVE_IN_ROOT},
    {"RESOLVE_NO_MAGICLINKS", PPM_RESOLVE_NO_MAGICLINKS},
    {"RESOLVE_NO_SYMLINKS", PPM_RESOLVE_NO_SYMLINKS},
    {"RESOLVE_NO_XDEV", PPM_RESOLVE_NO_XDEV},
    {"RESOLVE_CACHED", PPM_RESOLVE_CACHED},
    {0, 0},
};

const struct ppm_name_value execve_flags[] = {
    {"EXE_WRITABLE", PPM_EXE_WRITABLE},
    {"EXE_UPPER_LAYER", PPM_EXE_UPPER_LAYER},
    {"EXE_FROM_MEMFD", PPM_EXE_FROM_MEMFD},
    {0, 0},
};

const struct ppm_name_value execveat_flags[] = {
    {"AT_EMPTY_PATH", PPM_EXVAT_AT_EMPTY_PATH},
    {"AT_SYMLINK_NOFOLLOW", PPM_EXVAT_AT_SYMLINK_NOFOLLOW},
    {0, 0},
};

const struct ppm_name_value io_uring_setup_flags[] = {
    {"IORING_SETUP_IOPOLL", PPM_IORING_SETUP_IOPOLL},
    {"IORING_SETUP_SQPOLL", PPM_IORING_SETUP_SQPOLL},
    {"IORING_SQ_NEED_WAKEUP", PPM_IORING_SQ_NEED_WAKEUP},
    {"IORING_SETUP_SQ_AFF", PPM_IORING_SETUP_SQ_AFF},
    {"IORING_SETUP_CQSIZE", PPM_IORING_SETUP_CQSIZE},
    {"IORING_SETUP_CLAMP", PPM_IORING_SETUP_CLAMP},
    {"IORING_SETUP_ATTACH_RW", PPM_IORING_SETUP_ATTACH_WQ},
    {"IORING_SETUP_R_DISABLED", PPM_IORING_SETUP_R_DISABLED},
    {0,0},
};

const struct ppm_name_value io_uring_setup_feats[] = {
    {"IORING_FEAT_SINGLE_MMAP",PPM_IORING_FEAT_SINGLE_MMAP},
    {"IORING_FEAT_NODROP", PPM_IORING_FEAT_NODROP},
    {"IORING_FEAT_SUBMIT_STABLE", PPM_IORING_FEAT_SUBMIT_STABLE},
    {"IORING_FEAT_RW_CUR_POS", PPM_IORING_FEAT_RW_CUR_POS},
    {"IORING_FEAT_CUR_PERSONALITY", PPM_IORING_FEAT_CUR_PERSONALITY},
    {"IORING_FEAT_FAST_POLL", PPM_IORING_FEAT_FAST_POLL},
    {"IORING_FEAT_POLL_32BITS", PPM_IORING_FEAT_POLL_32BITS},
    {"IORING_FEAT_SQPOLL_NONFIXED", PPM_IORING_FEAT_SQPOLL_NONFIXED},
    {"IORING_FEAT_ENTER_EXT_ARG", PPM_IORING_FEAT_ENTER_EXT_ARG},
    {"IORING_FEAT_NATIVE_WORKERS", PPM_IORING_FEAT_NATIVE_WORKERS},
    {"IORING_FEAT_RSRC_TAGS", PPM_IORING_FEAT_RSRC_TAGS},
    {0,0},
};

const struct ppm_name_value io_uring_enter_flags[] = {
    {"IORING_ENTER_GETEVENTS", PPM_IORING_ENTER_GETEVENTS},
    {"IORING_ENTER_SQ_WAKEUP", PPM_IORING_ENTER_SQ_WAKEUP},
    {"IORING_ENTER_SQ_WAIT", PPM_IORING_ENTER_SQ_WAIT},
    {"IORING_ENTER_EXT_ARG", PPM_IORING_ENTER_EXT_ARG},
    {0,0},
};

const struct ppm_name_value io_uring_register_opcodes[] = {
    {"IORING_REGISTER_BUFFERS", PPM_IORING_REGISTER_BUFFERS},
    {"IORING_UNREGISTER_BUFFERS",PPM_IORING_UNREGISTER_BUFFERS},
    {"IORING_REGISTER_FILES",PPM_IORING_REGISTER_FILES},
    {"IORING_UNREGISTER_FILES", PPM_IORING_UNREGISTER_FILES},
    {"IORING_REGISTER_EVENTFD", PPM_IORING_REGISTER_EVENTFD},
    {"IORING_UNREGISTER_EVENTFD", PPM_IORING_UNREGISTER_EVENTFD},
    {"IORING_REGISTER_FILES_UPDATE", PPM_IORING_REGISTER_FILES_UPDATE},
    {"IORING_REGISTER_EVENTFD_ASYNC", PPM_IORING_REGISTER_EVENTFD_ASYNC},
    {"IORING_REGISTER_PROBE", PPM_IORING_REGISTER_PROBE},
    {"IORING_REGISTER_PERSONALITY", PPM_IORING_REGISTER_PERSONALITY},
    {"IORING_UNREGISTER_PERSONALITY", PPM_IORING_UNREGISTER_PERSONALITY},
    {"IORING_REGISTER_RESTRICTIONS", PPM_IORING_REGISTER_RESTRICTIONS},
    {"IORING_REGISTER_ENABLE_RINGS", PPM_IORING_REGISTER_ENABLE_RINGS},
    {"IORING_REGISTER_FILES2", PPM_IORING_REGISTER_FILES2},
    {"IORING_REGISTER_FILES_UPDATE2", PPM_IORING_REGISTER_FILES_UPDATE2},
    {"IORING_REGISTER_BUFFERS2", PPM_IORING_REGISTER_BUFFERS2},
    {"IORING_REGISTER_BUFFERS_UPDATE", PPM_IORING_REGISTER_BUFFERS_UPDATE},
    {"IORING_REGISTER_IOWQ_AFF", PPM_IORING_REGISTER_IOWQ_AFF},
    {"IORING_UNREGISTER_IOWQ_AFF", PPM_IORING_UNREGISTER_IOWQ_AFF},
    {"IORING_REGISTER_IOWQ_MAX_WORKERS", PPM_IORING_REGISTER_IOWQ_MAX_WORKERS},
    {"IORING_REGISTER_RING_FDS", PPM_IORING_REGISTER_RING_FDS},
    {"IORING_UNREGISTER_RING_FDS", PPM_IORING_UNREGISTER_RING_FDS},
    {0, 0}
};

const struct ppm_name_value mlockall_flags[] = {
    {"MCL_CURRENT", PPM_MLOCKALL_MCL_CURRENT},
    {"MCL_FUTURE", PPM_MLOCKALL_MCL_FUTURE},
    {"MCL_ONFAULT", PPM_MLOCKALL_MCL_ONFAULT},
    {0,0},
};

const struct ppm_name_value mlock2_flags[] = {
    {"MLOCK_ONFAULT", PPM_MLOCK_ONFAULT},
    {0,0},
};

const struct ppm_name_value fsconfig_cmds[] = {
    {"FSCONFIG_SET_FLAG", PPM_FSCONFIG_SET_FLAG},
    {"FSCONFIG_SET_STRING", PPM_FSCONFIG_SET_STRING},
    {"FSCONFIG_SET_BINARY", PPM_FSCONFIG_SET_BINARY},
    {"FSCONFIG_SET_PATH", PPM_FSCONFIG_SET_PATH},
    {"FSCONFIG_SET_PATH_EMPTY", PPM_FSCONFIG_SET_PATH_EMPTY},
    {"FSCONFIG_SET_FD", PPM_FSCONFIG_SET_FD},
    {"FSCONFIG_CMD_CREATE", PPM_FSCONFIG_CMD_CREATE},
    {"FSCONFIG_CMD_RECONFIGURE", PPM_FSCONFIG_CMD_RECONFIGURE},
    {0, 0},
};

const struct ppm_name_value epoll_create1_flags[] = {
    {"EPOLL_CLOEXEC", PPM_EPOLL_CLOEXEC},
    {0, 0},
};

const struct ppm_name_value prctl_options[] = {
    {"PR_GET_DUMPABLE",PPM_PR_GET_DUMPABLE},
    {"PR_SET_DUMPABLE",PPM_PR_SET_DUMPABLE},
    {"PR_GET_KEEPCAPS",PPM_PR_GET_KEEPCAPS},
    {"PR_SET_KEEPCAPS",PPM_PR_SET_KEEPCAPS},
    {"PR_SET_NAME",PPM_PR_SET_NAME},
    {"PR_GET_NAME",PPM_PR_GET_NAME},
    {"PR_GET_SECCOMP",PPM_PR_GET_SECCOMP},
    {"PR_SET_SECCOMP",PPM_PR_SET_SECCOMP},
    {"PR_CAPBSET_READ",PPM_PR_CAPBSET_READ},
    {"PR_CAPBSET_DROP",PPM_PR_CAPBSET_DROP},
    {"PR_GET_SECUREBITS",PPM_PR_GET_SECUREBITS},
    {"PR_SET_SECUREBITS",PPM_PR_SET_SECUREBITS},
    {"PR_MCE_KILL",PPM_PR_MCE_KILL},
    {"PR_SET_MM",PPM_PR_SET_MM},
    {"PR_SET_CHILD_SUBREAPER",PPM_PR_SET_CHILD_SUBREAPER},
    {"PR_GET_CHILD_SUBREAPER",PPM_PR_GET_CHILD_SUBREAPER},
    {"PR_SET_NO_NEW_PRIVS",PPM_PR_SET_NO_NEW_PRIVS},
    {"PR_GET_NO_NEW_PRIVS",PPM_PR_GET_NO_NEW_PRIVS},
    {"PR_GET_TID_ADDRESS",PPM_PR_GET_TID_ADDRESS},
    {"PR_SET_THP_DISABLE",PPM_PR_SET_THP_DISABLE},
    {"PR_GET_THP_DISABLE",PPM_PR_GET_THP_DISABLE},
    {"PR_CAP_AMBIENT",PPM_PR_CAP_AMBIENT},
    {0, 0},
};

const struct ppm_name_value memfd_create_flags[] = {
    {"MFD_CLOEXEC",PPM_MFD_CLOEXEC},
    {"MFD_ALLOW_SEALING",PPM_MFD_ALLOW_SEALING},
    {"MFD_HUGETLB",PPM_MFD_HUGETLB},
    {0,0},
};

const struct ppm_name_value pidfd_open_flags[] = {
    {"PIDFD_NONBLOCK", PPM_PIDFD_NONBLOCK},
    {0,0},
};

const struct ppm_name_value mknod_mode[] = {
    {"S_IXOTH", PPM_S_IXOTH},
    {"S_IWOTH", PPM_S_IWOTH},
    {"S_IROTH", PPM_S_IROTH},
    {"S_IXGRP", PPM_S_IXGRP},
    {"S_IWGRP", PPM_S_IWGRP},
    {"S_IRGRP", PPM_S_IRGRP},
    {"S_IXUSR", PPM_S_IXUSR},
    {"S_IWUSR", PPM_S_IWUSR},
    {"S_IRUSR", PPM_S_IRUSR},
    {"S_ISVTX", PPM_S_ISVTX},
    {"S_ISGID", PPM_S_ISGID},
    {"S_ISUID", PPM_S_ISUID},
    {"S_IFREG", PPM_S_IFREG},
    {"S_IFCHR", PPM_S_IFCHR},
    {"S_IFBLK", PPM_S_IFBLK},
    {"S_IFIFO", PPM_S_IFIFO},
    {"S_IFSOCK", PPM_S_IFSOCK},
    {0, 0},
};

const struct ppm_name_value bpf_commands[] = {
    {"BPF_MAP_CREATE", PPM_BPF_MAP_CREATE},
    {"BPF_MAP_LOOKUP_ELEM", PPM_BPF_MAP_LOOKUP_ELEM},
    {"BPF_MAP_UPDATE_ELEM", PPM_BPF_MAP_UPDATE_ELEM},
    {"BPF_MAP_DELETE_ELEM", PPM_BPF_MAP_DELETE_ELEM},
    {"BPF_MAP_GET_NEXT_KEY", PPM_BPF_MAP_GET_NEXT_KEY},
    {"BPF_PROG_LOAD", PPM_BPF_PROG_LOAD},
    {"BPF_OBJ_PIN", PPM_BPF_OBJ_PIN},
    {"BPF_OBJ_GET", PPM_BPF_OBJ_GET},
    {"BPF_PROG_ATTACH", PPM_BPF_PROG_ATTACH},
    {"BPF_PROG_DETACH", PPM_BPF_PROG_DETACH},
    {"BPF_PROG_TEST_RUN", PPM_BPF_PROG_TEST_RUN},
    {"BPF_PROG_RUN", PPM_BPF_PROG_RUN},
    {"BPF_PROG_GET_NEXT_ID", PPM_BPF_PROG_GET_NEXT_ID},
    {"BPF_MAP_GET_NEXT_ID", PPM_BPF_MAP_GET_NEXT_ID},
    {"BPF_PROG_GET_FD_BY_ID", PPM_BPF_PROG_GET_FD_BY_ID},
    {"BPF_MAP_GET_FD_BY_ID", PPM_BPF_MAP_GET_FD_BY_ID},
    {"BPF_OBJ_GET_INFO_BY_FD", PPM_BPF_OBJ_GET_INFO_BY_FD},
    {"BPF_PROG_QUERY", PPM_BPF_PROG_QUERY},
    {"BPF_RAW_TRACEPOINT_OPEN", PPM_BPF_RAW_TRACEPOINT_OPEN},
    {"BPF_BTF_LOAD", PPM_BPF_BTF_LOAD},
    {"BPF_BTF_GET_FD_BY_ID", PPM_BPF_BTF_GET_FD_BY_ID},
    {"BPF_TASK_FD_QUERY", PPM_BPF_TASK_FD_QUERY},
    {"BPF_MAP_LOOKUP_AND_DELETE_ELEM", PPM_BPF_MAP_LOOKUP_AND_DELETE_ELEM},
    {"BPF_MAP_FREEZE", PPM_BPF_MAP_FREEZE},
    {"BPF_BTF_GET_NEXT_ID", PPM_BPF_BTF_GET_NEXT_ID},
    {"BPF_MAP_LOOKUP_BATCH", PPM_BPF_MAP_LOOKUP_BATCH},
    {"BPF_MAP_LOOKUP_AND_DELETE_BATCH", PPM_BPF_MAP_LOOKUP_AND_DELETE_BATCH},
    {"BPF_MAP_UPDATE_BATCH", PPM_BPF_MAP_UPDATE_BATCH},
    {"BPF_MAP_DELETE_BATCH", PPM_BPF_MAP_DELETE_BATCH},
    {"BPF_LINK_CREATE", PPM_BPF_LINK_CREATE},
    {"BPF_LINK_UPDATE", PPM_BPF_LINK_UPDATE},
    {"BPF_LINK_GET_FD_BY_ID", PPM_BPF_LINK_GET_FD_BY_ID},
    {"BPF_LINK_GET_NEXT_ID", PPM_BPF_LINK_GET_NEXT_ID},
    {"BPF_ENABLE_STATS", PPM_BPF_ENABLE_STATS},
    {"BPF_ITER_CREATE", PPM_BPF_ITER_CREATE},
    {"BPF_LINK_DETACH", PPM_BPF_LINK_DETACH},
    {"BPF_PROG_BIND_MAP", PPM_BPF_PROG_BIND_MAP},
    {0,0},
};

const struct ppm_name_value delete_module_flags[] = {
    {"O_NONBLOCK", PPM_DELETE_MODULE_O_NONBLOCK},
    {"O_TRUNC", PPM_DELETE_MODULE_O_TRUNC},
    {0, 0},
};

const struct ppm_name_value finit_module_flags[] = {
    {"MODULE_INIT_IGNORE_MODVERSIONS", PPM_MODULE_INIT_IGNORE_MODVERSIONS},
    {"MODULE_INIT_IGNORE_VERMAGIC", PPM_MODULE_INIT_IGNORE_VERMAGIC},
    {"MODULE_INIT_COMPRESSED_FILE", PPM_MODULE_INIT_COMPRESSED_FILE},
    {0, 0},
};
}
