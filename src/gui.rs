use std::{rc::Rc, cell::RefCell};
use native_windows_gui as nwg;
use winapi::um::winuser::{
    SetTimer, KillTimer, WS_EX_TOOLWINDOW
};
use winapi::um::dwmapi::DwmSetWindowAttribute;
use winapi::shared::windef::HWND;

use crate::{autostart, events, utils};

const DWMWA_WINDOW_CORNER_PREFERENCE: u32 = 33;
const WIDGET_SIZE: (i32, i32) = (250, 75);
const BACKGROUND_COLOR: Option<[u8; 3]> = Some([152, 251, 152]);

static ICONO_BYTES: &[u8] = include_bytes!("../icono.ico");

pub struct UI {
    #[allow(dead_code)]
    pub background_frame: nwg::RichLabel,
    #[allow(dead_code)]
    pub image: nwg::Bitmap,
    #[allow(dead_code)]
    pub image_control: nwg::ImageFrame,
    pub text_label: nwg::RichLabel,
    pub countdown_label: nwg::RichLabel,
    pub window: nwg::Window,
}

pub struct Tray {
    pub tray: nwg::TrayNotification,
    pub tray_menu: nwg::Menu,
    pub mostrar_item: nwg::MenuItem,
    pub cerrar_item: nwg::MenuItem,
    pub autostart_item: nwg::MenuItem
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    nwg::init()?;

    let position = utils::load_position();

    let mut window = nwg::Window::default();
    nwg::Window::builder()
        .size(WIDGET_SIZE)
        .position(position)
        .title("Aniversario 100 - EELC")
        .flags(nwg::WindowFlags::POPUP | nwg::WindowFlags::VISIBLE)
        .ex_flags(WS_EX_TOOLWINDOW)
        .build(&mut window)?;
    window.set_visible(true);

    round_window(&window);
    start_timer(&window);

    let mut ui = create_ui(&window)?;
    ui.window = window;

    let tray = Rc::new(RefCell::new(create_tray(&ui.window)?));
    let ui = Rc::new(RefCell::new(ui));

    events::attach_event_handler(ui.clone(), tray.clone());

    nwg::dispatch_thread_events();

    Ok(())
}


fn create_ui(window: &nwg::Window) -> Result<UI, Box<dyn std::error::Error>> {
    let mut background_frame = nwg::RichLabel::default();
    nwg::RichLabel::builder()
        .text("")
        .position((0, 0))
        .background_color(BACKGROUND_COLOR)
        .size((250, 75))
        .parent(window)
        .build(&mut background_frame)?;

    let image_data = include_bytes!("../imagen-icono.png");
    let mut image = nwg::Bitmap::default();
    nwg::Bitmap::builder()
        .source_bin(Some(image_data))
        .build(&mut image)?;

    let mut image_control = nwg::ImageFrame::default();
    nwg::ImageFrame::builder()
        .bitmap(Some(&image))
        .position((18, 14))
        .size((48, 48))
        .parent(&background_frame)
        .build(&mut image_control)?;

    let mut font = nwg::Font::default();
    nwg::Font::builder()
        .family("Segoe UI")
        .size(21)
        .weight(600)
        .build(&mut font)?;

    let mut text_label = nwg::RichLabel::default();
    nwg::RichLabel::builder()
        .position((70, 14))
        .size((160, 25))
        .font(Some(&font))
        .background_color(BACKGROUND_COLOR)
        .text("Aniversario 100 años")
        .parent(&background_frame)
        .build(&mut text_label)?;

    let mut countdown_label = nwg::RichLabel::default();
    nwg::RichLabel::builder()
        .text("Loading ...")
        .font(Some(&font))
        .position((70, 38))
        .size((160, 25))
        .background_color(BACKGROUND_COLOR)
        .parent(&background_frame)
        .build(&mut countdown_label)?;

    Ok(UI {
        background_frame,
        image,
        image_control,
        text_label,
        countdown_label,
        window: nwg::Window::default(), // será asignado después
    })
}


fn create_tray(window: &nwg::Window) -> Result<Tray, Box<dyn std::error::Error>> {
    let mut icon = nwg::Icon::default();
    nwg::Icon::builder()
        .source_bin(Some(ICONO_BYTES))
        .build(&mut icon)?;

    let mut tray = nwg::TrayNotification::default();
    nwg::TrayNotification::builder()
        .icon(Some(&icon))
        .parent(window)
        .build(&mut tray)?;

    let mut tray_menu = nwg::Menu::default();
    let mut mostrar_item = nwg::MenuItem::default();
    let mut autostart_item = nwg::MenuItem::default();
    let mut cerrar_item = nwg::MenuItem::default();

    nwg::Menu::builder()
        .popup(true)
        .parent(window)
        .build(&mut tray_menu)?;

    nwg::MenuItem::builder()
        .text("Mostrar/Ocultar")
        .parent(&tray_menu)
        .build(&mut mostrar_item)?;

    nwg::MenuItem::builder()
        .text("Iniciar con Windows")
        .parent(&tray_menu)
        .build(&mut autostart_item)?;
    autostart_item.set_checked(autostart::is_enabled());

    nwg::MenuItem::builder()
        .text("Cerrar")
        .parent(&tray_menu)
        .build(&mut cerrar_item)?;

    Ok(Tray {
        tray,
        tray_menu,
        mostrar_item,
        cerrar_item,
        autostart_item,
    })
}


fn round_window(window: &nwg::Window) {
    unsafe {
        let hwnd = window.handle.hwnd().unwrap() as HWND;
        let preference: u32 = 2; // DWMWCP_ROUND
        DwmSetWindowAttribute(
            hwnd,
            DWMWA_WINDOW_CORNER_PREFERENCE,
            &preference as *const u32 as *const _,
            std::mem::size_of::<u32>() as u32,
        );
    }
}


pub fn toggle_timer(window_handle: nwg::ControlHandle, start: bool) {
    unsafe {
        let hwnd = window_handle.hwnd().unwrap() as HWND;
        if start {
            SetTimer(hwnd, 1, 1000, None);
        } else {
            KillTimer(hwnd, 1);
        }
    }
}


fn start_timer(window: &nwg::Window) {
    toggle_timer(window.handle, true);
}


pub fn create_notification(days: i64, tray: &nwg::TrayNotification) {
    let text = match days {
        30 => Some("🎉¡Queda un mes para el gran aniversario!🎉"),
        7 => Some("⏳¡Queda solo 1 semana para celebrar los 100 años!⏳"),
        0 => Some("🎊¡Mañana es el gran aniversario de los 100 años! 🎊"),
        100 => Some("🎉 ¡Feliz Aniversario de 100 AÑOS! 🥳"),
        _ => None,
    };
    let mut icon = nwg::Icon::default();
    nwg::Icon::builder()
        .source_bin(Some(ICONO_BYTES))
        .build(&mut icon)
        .expect("Error icono notificacion");

    tray.show("Aniversario 100 años - Esperanza en la Ciudad", Some(text.expect("Error")), None, Some(&icon));
}
