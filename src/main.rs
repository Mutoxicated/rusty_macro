#![feature(let_chains)]

use product::dresser::Dresser;

mod csmacro;
mod product;
mod helpers;

fn main() {
    egui_options!(options);

    eframe::run_native(
        "LOC Counter", 
        options, 
        Box::new(|_| {
            Box::<Dresser>::new(Dresser::new())
        })
    ).unwrap();
}
