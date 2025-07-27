use winres;

fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("icono.ico"); // Ruta relativa al icono
    res.compile().unwrap();
}