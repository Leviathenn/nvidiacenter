/**
 * @author Leviathenn
 */

fn main() {
    cc::Build::new()
        .warnings(false)
        .extra_warnings(false)
        .file("lib/caccess.c")
        .compile("caccess");
}