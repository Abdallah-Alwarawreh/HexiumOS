use crate::writer::Writer;
use crate::vga_colors::Color;
use crate::idt;

pub struct HexFetch {
}

impl HexFetch {

pub fn fetch(writer: &mut Writer){
    writer.set_color(Color::LightCyan, Color::Black);
    writer.write_str("  _    _           _                  ____   _____ ");

    writer.write_str(" OS: HexiumOS\n");
  
    writer.write_str(" | |  | |         (_)                / __ \\ / ____|");

    writer.write_str(" Version: N/A \n");
  
    writer.write_str(" | |__| | _____  ___ _   _ _ __ ___ | |  | | (___  \n");
    writer.write_str(" |  __  |/ _ \\ \\/ / | | | | '_ ` _ \\| |  | |\\___ \\ \n");
    writer.write_str(" | |  | |  __/>  <| | |_| | | | | | | |__| |____) |\n");
    writer.write_str(" |_|  |_|\\___/_/\\_\\_|\\__,_|_| |_| |_|\\____/|_____/ \n");
    writer.write_str("                                               \n");
    writer.write_str("                                               \n");

    
  
}



}
