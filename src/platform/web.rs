use std::io;
const LIMIT: usize = 65536;
// The matching implementation is registered in web/host.js before the WASM module starts.
extern "C" {
    fn hub_read(slot: u32, out: *mut u8, capacity: usize) -> i32;
    fn hub_write(slot: u32, data: *const u8, len: usize) -> i32;
    fn hub_buttons(player: usize) -> u32;
    fn hub_edges(player: usize) -> u32;
    fn hub_pad_count() -> usize;
    fn hub_scene(scene: u32, players: usize, en: bool);
}
pub fn read(slot: u32) -> io::Result<Option<Vec<u8>>> {
    let n = unsafe { hub_read(slot, std::ptr::null_mut(), 0) };
    if n == -1 {
        return Ok(None);
    }
    if n < 0 || n as usize > LIMIT {
        return Err(io::Error::other("Invalid or unavailable local save"));
    }
    let mut buf = vec![0; n as usize];
    if unsafe { hub_read(slot, buf.as_mut_ptr(), buf.len()) } != n {
        return Err(io::Error::other("Could not read save"));
    }
    Ok(Some(buf))
}
pub fn write(slot: u32, b: &[u8]) -> io::Result<()> {
    if b.len() > LIMIT || unsafe { hub_write(slot, b.as_ptr(), b.len()) } != 1 {
        return Err(io::Error::other("Could not write local save"));
    }
    Ok(())
}
pub fn input(p: usize) -> (u32, u32) {
    unsafe { (hub_buttons(p), hub_edges(p)) }
}
pub fn pads() -> usize {
    unsafe { hub_pad_count() }
}
pub fn scene(s: u32, n: usize, en: bool) {
    unsafe {
        hub_scene(s, n, en);
    }
}
