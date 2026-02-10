use crate::writer::Writer;
use crate::vga_colors::Color;
use crate::idt;

pub struct HexFetch {
}

impl HexFetch {

pub fn fetch(writer: &mut Writer){
writer.write_str("OS: Hexium OS x86_64\n");
writer.write_str("Other stuff lol idk :3\n");

}



}
