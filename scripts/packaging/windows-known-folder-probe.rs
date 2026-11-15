use std::ffi::c_void;

#[repr(C)]
struct Guid {
    a: u32,
    b: u16,
    c: u16,
    d: [u8; 8],
}

#[link(name = "shell32")]
extern "system" {
    fn SHGetKnownFolderPath(id: *const Guid, flags: u32, token: *mut c_void, path: *mut *mut u16) -> i32;
}
#[link(name = "ole32")]
extern "system" {
    fn CoTaskMemFree(p: *mut c_void);
}

fn query(id: Guid) -> u32 {
    unsafe {
        let mut path = std::ptr::null_mut();
        let result = SHGetKnownFolderPath(&id, 0, std::ptr::null_mut(), &mut path);
        if !path.is_null() {
            CoTaskMemFree(path.cast());
        }
        result as u32
    }
}

fn main() {
    let local = Guid {
        a: 0xf1b32785, b: 0x6fba, c: 0x4fcf,
        d: [0x9d, 0x55, 0x7b, 0x8e, 0x7f, 0x15, 0x70, 0x91],
    };
    let roaming = Guid {
        a: 0x3eb685db, b: 0x65f9, c: 0x4cf6,
        d: [0xa0, 0x3a, 0xe3, 0xef, 0x65, 0x72, 0x9f, 0x3d],
    };
    println!("local=0x{:08X} roaming=0x{:08X}", query(local), query(roaming));
}
