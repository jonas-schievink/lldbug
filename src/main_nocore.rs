#![feature(no_core, lang_items, start)]

#![no_core]

#[lang = "sized"]
trait Sized {}

#[lang = "copy"]
trait Copy {}

#[lang = "freeze"]
trait Freeze {}

#[start]
fn main(_: isize, _: *const *const u8) -> isize {
	0
}
