extern crate cc;

fn main() {
    cc::Build::new().file("limbs.c").compile("limbs");
}
