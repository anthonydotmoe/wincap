use std::{rc::Rc, cell::RefCell};

use winsafe::{prelude::*, gui, co, HWND};

#[derive(Clone)]
pub struct ScreenView {
    wnd: gui::WindowControl,
    cap_rect: Rc<RefCell<winsafe::RECT>>,
}

impl ScreenView {
    pub fn new(parent: &impl GuiParent, position: (i32, i32), size: (u32, u32)) -> Self {
        let wnd = gui::WindowControl::new(
            parent,
            gui::WindowControlOpts{
                position,
                size,
                style: co::WS::CHILD | co::WS::VISIBLE,
                ex_style: co::WS_EX::TRANSPARENT,
                resize_behavior: (gui::Horz::Resize, gui::Vert::Resize),
                ..Default::default()
            }
        );
        
        let new_self = Self {
            wnd,
            cap_rect: Rc::new(RefCell::new(winsafe::RECT::default())),
        };

        new_self.events();
        new_self
    }
    
    pub fn redraw_screen(&self) -> winsafe::SysResult<()> {
        self.wnd.hwnd().InvalidateRect(None, true)
    }
    
    fn events(&self) {
        
        /*
         * WM_SIZE
         */

        let self2 = self.clone();
        self.wnd.on().wm_size(move |p| {
            let mut rect = self2.cap_rect.borrow_mut();

            let dlg_rect = self2.wnd.hwnd().GetParent().unwrap().GetWindowRect().unwrap();
            let capt_rect = self2.wnd.hwnd().GetWindowRect().unwrap();
            
            println!("Size: left: {}, top: {}, right: {}, bottom: {}",
                capt_rect.left,
                capt_rect.top,
                capt_rect.right,
                capt_rect.bottom,
            );
            
            rect.left = capt_rect.left;
            rect.top = capt_rect.top;
            rect.right = capt_rect.right;
            rect.bottom = capt_rect.bottom;
            
            
            self2.wnd.hwnd().InvalidateRect(None, true)?;
            
            Ok(())
        });

        /*
         * WM_PAINT
         */
        
        let self2 = self.clone();
        self.wnd.on().wm_paint(move || {
            let hdc = self2.wnd.hwnd().BeginPaint()?;
            
            let pen = winsafe::HPEN::CreatePen(
                co::PS::SOLID, 1, winsafe::COLORREF::new(0, 0, 0)
            )?;

            let rect = self2.cap_rect.borrow();
            
            let brush = winsafe::HBRUSH::CreateSolidBrush(winsafe::COLORREF::new(0, 255, 0))?;
            
            //let _old_brush = hdc.SelectObject(&*brush);
            
            let fill_rect = winsafe::RECT {
                left: 0,
                top: 0,
                right: rect.right - rect.left,
                bottom: rect.bottom - rect.top,
            };
            
            hdc.FillRect(fill_rect, &brush)?;

            let _old_pen = hdc.SelectObject(&*pen);

            hdc.MoveToEx(0, 0, None)?;

            hdc.LineTo(rect.right-rect.left-1, 0)?;
            hdc.LineTo(rect.right-rect.left-1, rect.bottom-rect.top-1)?;
            hdc.LineTo(0, rect.bottom-rect.top-1)?;
            hdc.LineTo(0, 0)?;
            
            Ok(())
        });
        
        /*
         * WM_CREATE

        ku
        let self2 = self.clone();
        self.wnd.on().wm_create(move |c| {
            self2.wnd.hwnd().SetTimer(1, 30, None).unwrap();
            Ok(0)
        });
        
        let self2 = self.clone();
        self.wnd.on().wm_timer(1, move || {
            println!("Timer!");
            
            
            let hdc_desktop = HWND::DESKTOP.GetDC()?;
            let hdc_view = self2.wnd.hwnd().GetDC()?;
            
            let rect = self2.cap_rect.borrow();
            
            let view_w = rect.right - rect.left - 2;
            let view_h = rect.bottom - rect.top - 2;

            hdc_view.BitBlt(
                winsafe::POINT{x: 1, y: 1},
                winsafe::SIZE{cx: view_w.into(), cy: view_h.into()},
                &hdc_desktop,
                winsafe::POINT{x: rect.left+1, y: rect.top+1},
                co::ROP::SRCCOPY
            )?;
            
            Ok(())

        });
         */
    }
}