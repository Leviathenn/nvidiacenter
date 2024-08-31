/**
 * @author Leviathenn
 * 
 */

#[repr(C)]
pub struct Caccess {
    pub is_root: i32,
}
extern "C" {
    pub fn is_root() -> i32;
}