pub use self::arch::{netent};
pub use self::arch::{hostent};
pub use self::arch::{servent};
pub use self::arch::{protoent};
pub use self::arch::{addrinfo};
pub use self::arch::{IPPORT_RESERVED};
pub use self::arch::{AI_PASSIVE};
pub use self::arch::{AI_CANONNAME};
pub use self::arch::{AI_NUMERICHOST};
pub use self::arch::{AI_NUMERICSERV};
pub use self::arch::{AI_V4MAPPED};
pub use self::arch::{AI_ALL};
pub use self::arch::{AI_ADDRCONFIG};
pub use self::arch::{NI_NOFQDN};
pub use self::arch::{NI_NUMERICHOST};
pub use self::arch::{NI_NAMEREQD};
pub use self::arch::{NI_NUMERICSERV};
pub use self::arch::{NI_DGRAM};
pub use self::arch::{EAI_AGAIN};
pub use self::arch::{EAI_BADFLAGS};
pub use self::arch::{EAI_FAIL};
pub use self::arch::{EAI_FAMILY};
pub use self::arch::{EAI_MEMORY};
pub use self::arch::{EAI_NONAME};
pub use self::arch::{EAI_SERVICE};
pub use self::arch::{EAI_SOCKTYPE};
pub use self::arch::{EAI_SYSTEM};
pub use self::arch::{EAI_OVERFLOW};

#[cfg(target_arch = "x86_64")]
#[path = "x86_64.rs"]
mod arch;

#[cfg(target_arch = "x86")]
#[path = "x86.rs"]
mod arch;

