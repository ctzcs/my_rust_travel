use std::error::Error;
use tetra::Context;
pub mod sample_mode;


pub trait IMode {
    fn ui(&mut self, ctx: &mut tetra::Context,egui_ctx: &egui::Context) -> Result<(), Box<dyn Error>>;
    fn update(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>>;
    fn draw(&mut self, ctx: &mut Context) -> Result<(), Box<dyn Error>>;
    fn event(&mut self, ctx: &mut Context,event: tetra::Event) -> Result<(), Box<dyn Error>>;
}
