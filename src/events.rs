use native_windows_gui as nwg;
use winapi::um::winuser::{GetCursorPos, ReleaseCapture, SendMessageW, WM_NCLBUTTONDOWN, HTCAPTION};
use std::{rc::Rc, cell::RefCell};
use winapi::shared::windef::{HWND, POINT};
use chrono::{DateTime, Duration, TimeZone, Utc};
use chrono_tz::America::Montevideo;

use crate::{autostart, gui, utils};

pub fn attach_event_handler(ui: Rc<RefCell<gui::UI>>, tray: Rc<RefCell<gui::Tray>>) {
    let window_handle = ui.borrow().window.handle;

    nwg::full_bind_event_handler(&window_handle, move |evt, _evt_data, handle| {
        match evt {
            nwg::Event::OnMousePress(_) => { move_window(&window_handle); }

            nwg::Event::OnContextMenu => {
                if handle == tray.borrow().tray.handle { show_menu(&tray.borrow().tray_menu); }
            }

            nwg::Event::OnMenuItemSelected => {
                let tray_ref = tray.borrow();

                if handle == tray_ref.mostrar_item.handle { show_window(&ui); }
                if handle == tray_ref.cerrar_item.handle { close_window(&ui.borrow().window); }
                if handle == tray_ref.autostart_item.handle { autostart_app(&tray_ref.autostart_item); }
            }

            nwg::Event::OnTimerTick => { update_countdown_label(&ui, &tray); }

            nwg::Event::OnWindowClose => { if handle == ui.borrow().window.handle { utils::save_position(ui.borrow().window.position()); } }

            _ => {}
        }
    });
}


fn move_window(window_handle: &nwg::ControlHandle) {
    unsafe {
        let hwnd = window_handle.hwnd().unwrap() as HWND;
        ReleaseCapture();
        SendMessageW(hwnd, WM_NCLBUTTONDOWN, HTCAPTION as usize, 0);
    }
}


fn show_menu(menu: &nwg::Menu) {
    unsafe {
        let mut point: POINT = std::mem::zeroed();
        if GetCursorPos(&mut point) != 0 {
            nwg::Menu::popup(menu, point.x, point.y);
        }
    }
}


fn show_window(ui: &Rc<RefCell<gui::UI>>) {
    let ui_mut = ui.borrow_mut();
    let visible = !ui_mut.window.visible();

    ui_mut.window.set_visible(visible);
    crate::gui::toggle_timer(ui_mut.window.handle, visible);
}


fn close_window(window: &nwg::Window) {
    crate::gui::toggle_timer(window.handle, false);
    utils::save_position(window.position());
    nwg::stop_thread_dispatch();
}


fn autostart_app(autostart_item: &nwg::MenuItem) {
    let enabled = !autostart::is_enabled();
    autostart::set_enabled(enabled);
    autostart_item.set_checked(enabled);
}


fn update_countdown_label(ui: &Rc<RefCell<gui::UI>>, tray: &Rc<RefCell<gui::Tray>>) {
    let date = Montevideo.with_ymd_and_hms(2026, 7, 4, 0, 0, 0).unwrap();
    let text = countdown_time(date, &tray.borrow_mut().tray);

    ui.borrow_mut().countdown_label.set_text(&text);
    if text == "Esperanza en la Ciudad" {
        ui.borrow_mut().text_label.set_text("¡Feliz Aniversario!");
    }
}


fn countdown_time(date: DateTime<chrono_tz::Tz>, tray: &nwg::TrayNotification) -> String {
    let now = Utc::now();
    let date_utc = date.with_timezone(&Utc);
    let duration = date_utc - now;

    if duration < Duration::zero() {
        "Esperanza en la Ciudad".to_string()
    } else {
        let total_seconds = duration.num_seconds();
        let days = total_seconds / 86_400;
        let hours = (total_seconds % 86_400) / 3600;
        let minutes = (total_seconds % 3600) / 60;
        let seconds = total_seconds % 60;

        match days {
            30 | 7 | 0 => {
                let notified = utils::is_notificated(days);
                if !notified {
                    utils::save_notification(days);
                    gui::create_notification(days, tray);
                }
            }
            _ => {}
        }

        if days > 0 {
            format!("{} días, {:02}:{:02}:{:02}", days, hours, minutes, seconds)
        } else {
            format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
        }
    }
}
