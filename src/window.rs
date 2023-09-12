use winsafe::{gui, prelude::*, co};

use crate::screen_view::ScreenView;
use crate::resource_consts::{*, self};

#[derive(Clone)]
pub struct Window {
    wnd: gui::WindowMain,
    x_res_edit: gui::Edit,
    y_res_edit: gui::Edit,
    record_btn: gui::Button,
    stop_btn: gui::Button,
    size_lbl: gui::Label,
    x_lbl: gui::Label,
    screen_view: ScreenView,
}

impl Window {
    pub fn new() -> Self {
        let wnd = gui::WindowMain::new_dlg(
            IDD_MAIN, None, None);
        
        let screen_view = ScreenView::new(
            &wnd,
            (4, 4),
            (543, 314),
        );
        
        let x_res_edit = gui::Edit::new_dlg(&wnd, IDC_XSZ, (gui::Horz::None, gui::Vert::Repos));
        let y_res_edit = gui::Edit::new_dlg(&wnd, IDC_YSZ, (gui::Horz::None, gui::Vert::Repos));
        let record_btn = gui::Button::new_dlg(&wnd, IDC_REC, (gui::Horz::Repos, gui::Vert::Repos));
        let stop_btn = gui::Button::new_dlg(&wnd, IDC_STOP, (gui::Horz::Repos, gui::Vert::Repos));
        let size_lbl = gui::Label::new_dlg(&wnd, IDC_SIZE_LBL, (gui::Horz::None, gui::Vert::Repos));
        let x_lbl = gui::Label::new_dlg(&wnd, IDC_X_LBL, (gui::Horz::None, gui::Vert::Repos));

        let mut new_self = Self {
            wnd,
            x_res_edit,
            y_res_edit,
            record_btn,
            stop_btn,
            size_lbl,
            x_lbl,
            screen_view,
        };
        new_self.events();
        new_self
    }
    
    pub fn run(&self) -> winsafe::AnyResult<i32> {
        self.wnd.run_main(None)
    }
    
    fn events(&self) {
        let self2 = self.clone();

        self.record_btn.on().bn_clicked(move || {
            self2.x_res_edit.set_text("Button has been pressed.");
            
            Ok(())
        });
        
        let self2 = self.clone();
        self.wnd.on().wm_moving(move |_| {
            self2.screen_view.redraw_screen();
            Ok(())
        });
        
        /*
        let self2 = self.clone();
        self.wnd.on().wm_init_dialog(move |i| {
            self2.wnd.hwnd().SetTimer(1, 30, None).unwrap();
            Ok(true)
        });
        
        let self2 = self.clone();
        self.wnd.on().wm_timer(1, move || {
            
            println!("Timer!");
            
            Ok(())

        });
        */
    }
}