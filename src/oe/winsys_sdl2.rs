use std::sync::Arc;
use no_deadlocks::Mutex;
extern crate sdl2 as sdl2;
extern crate gl;

use nohash_hasher::IntMap;
//use sdl2::event::{EventWatch, EventWatchCallback};


use super::base_traits::*;
use super::dummy_structs::*;
use super::event_handler::*;
//use super::global_variables::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ButtonState {
    Released,
    JustPressed,
    Pressed,
    JustReleased,
}
#[derive(Clone)]
pub struct WinsysEventPumpSdl2<'a>{
    data_       : Arc<Mutex<WinsysOutput>>,
    event_handler : UltimateWrapper<EventHandler<'a>>,

    keyboard_events_map : IntMap<i32, [usize; 3]>,
    mouse_events_map : IntMap<i32, [usize; 3]>,
}

/*struct WinsysEventCallbackSdl2<'a>{
    pump : Arc<WinsysEventPumpSdl2<'a>>,
}

impl<'a> EventWatchCallback for WinsysEventCallbackSdl2<'a>{
    fn callback(&mut self, event : sdl2::event::Event){
        self.pump.handle_event(event);
    }
}*/
pub struct WinsysSdl2<'a>{
    sdl_ctx     : sdl2::Sdl,
    sdlvideo    : sdl2::VideoSubsystem,
    window_     : sdl2::video::Window,
    context_    : sdl2::video::GLContext,
    sdl_pump_   : sdl2::EventPump, 
    sdl_events_ : sdl2::EventSubsystem,
    //sdl_ewatch_ : Arc<EventWatch<'a, WinsysEventCallbackSdl2<'a>>>,
    event_pump_ : Arc<WinsysEventPumpSdl2<'a>>,
    //event_pump_ : sdl2::EventPump,

    //data          : WinsysOutput,
}


impl<'a> WinsysSdl2<'a>{
    pub fn new(init_info: &WinsysInitInfo, update_info: &WinsysUpdateInfo, event_handler : UltimateWrapper<EventHandler<'a>>) -> Self{
        
        let sdl = sdl2::init().unwrap();
        
        if init_info.requested_backend == WinsysBackend::Angle{
            sdl2::hint::set("SDL_VIDEO_WIN_D3DCOMPILER", "d3dcompiler_43.dll");
            sdl2::hint::set("SDL_OPENGL_ES_DRIVER", "1");
        }
        let video_sys = sdl.video().unwrap();

        let gl_attr = video_sys.gl_attr();
        let major;let minor;
        if init_info.requested_backend == WinsysBackend::Gl3 {
            gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
            gl_attr.set_context_version(3, 3);
            major=3;minor=3;
        }
        else if init_info.requested_backend == WinsysBackend::Gles2{
            gl_attr.set_context_profile(sdl2::video::GLProfile::GLES);
            gl_attr.set_context_version(2, 0);
            major=2;minor=0;
        }
        else if init_info.requested_backend == WinsysBackend::Angle{
            gl_attr.set_context_profile(sdl2::video::GLProfile::GLES);
            gl_attr.set_context_version(2, 0);
            major=2;minor=0;
        }
        else{
            major=1;minor=4;
        }
        gl_attr.set_accelerated_visual(true);
        gl_attr.set_double_buffer(true);

        let my_window = video_sys
                    .window(update_info.title.as_str(), update_info.res_x, update_info.res_y)
                    .opengl()
                    .resizable()
                    .build().unwrap();

        if update_info.use_fullscreen {
           //my_window.set_fullscreen(1).unwrap();
        }

        let my_context = my_window.gl_create_context().unwrap();

        gl::load_with(|s| video_sys.gl_get_proc_address(s) as *const std::os::raw::c_void);
        //gl::load_with(|s| video_sys.gl_get_proc_address(s).unwrap() as *const std::os::raw::c_void);
    
        unsafe {
            gl::ClearColor(0.3, 0.3, 0.5, 1.0);
        }
        let event_sys = sdl.event().unwrap();
        let sdl_pump = sdl.event_pump().unwrap();


        // INITIALIZE KEYBOARD AND MOUSE EVENTS
        let mut keyboard_events_map : IntMap<i32, [usize; 3]> = Default::default();
        let mut mouse_events_map : IntMap<i32, [usize; 3]> = Default::default();
        {
            let mut eh = event_handler.write().unwrap();
            for (sc, _) in sdl_pump.keyboard_state().scancodes() {
                let event_name = &format!{"{sc:?}"};
                let mut event_ids = [0, 0, 0];
                for (i, suffix) in ["+", "", "-"].iter().enumerate() {
                    let event_id = eh.as_mut().unwrap().create_keyboard_event(&(event_name.to_string() + suffix));
                    event_ids[i] = event_id;

                }
                keyboard_events_map.insert(sc as i32, event_ids);
            }
            for (sc, _) in sdl_pump.mouse_state().mouse_buttons() {
                let event_name = &format!{"{sc:?}"};
                let mut event_ids = [0, 0, 0];
                for (i, suffix) in ["+", "", "-"].iter().enumerate() {
                    let event_id = eh.as_mut().unwrap().create_mouse_event(&(event_name.to_string() + suffix));
                    event_ids[i] = event_id;

                }
                mouse_events_map.insert(sc as i32, event_ids);
            }
            //eh.as_mut().unwrap()
            eh.as_mut().unwrap().create_mouse_event("motion");
            eh.as_mut().unwrap().create_mouse_event("lock");
            eh.as_mut().unwrap().create_mouse_event("unlock");
        }
        //-----------------

        let output = WinsysOutput {
            update_info : update_info.clone(),
            major,
            minor,
            backend : init_info.requested_backend,
            mouse_moved : false,
            done : false,
            dpi : 96,
        };

        let event_pump_ = Arc::new(WinsysEventPumpSdl2 {
            data_ : Arc::new(Mutex::new(output)),
            event_handler : event_handler,
            keyboard_events_map,
            mouse_events_map,
        });
        //let event_callback = WinsysEventCallbackSdl2{pump:Arc::clone(&event_pump_)};
        //let mut event_watch = event_sys.add_event_watch(event_callback);
        //event_watch.set_activated(true);
        //let wait_iter = Arc::new(Mutex::new(Some(event_pump.wait_iter())));
        WinsysSdl2{
            sdl_ctx  : sdl,
            sdlvideo : video_sys,
            window_  : my_window,
            context_ : my_context,
            sdl_pump_ : sdl_pump,
            sdl_events_: event_sys,
            //sdl_ewatch_: Arc::new(event_watch),
            event_pump_
        }
    }
}

impl WinsysBaseTrait for WinsysSdl2<'_>{

    fn update_events_single_thread(&mut self){
        for event in self.sdl_pump_.poll_iter(){
            let event_pump = &self.event_pump_;
            event_pump.handle_event(event);
        }
    }

    fn is_done(&self) -> bool {
        let output = self.event_pump_.data_.lock().unwrap();
        output.done 
    }

    fn update_window(&mut self, update_info : WinsysUpdateInfo) -> WinsysOutput {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        self.window_.gl_swap_window();
        let event_pump = &self.event_pump_;
        let mut data = event_pump.data_.lock().unwrap();
        if data.update_info.mouse_locked != update_info.mouse_locked {
            self.sdl_ctx.mouse().set_relative_mouse_mode(update_info.mouse_locked);
            data.update_info.mouse_locked = update_info.mouse_locked;
        }
        if data.update_info.title != update_info.title{
            self.window_.set_title(&update_info.title).unwrap();
            data.update_info.title = update_info.title;
        }
        if ! data.update_info.res_changed{
            data.update_info.res_x = update_info.res_x;
            data.update_info.res_y = update_info.res_y;
        }
        //data.update_info = update_info;
        data.update_info.res_changed = false;
        data.clone()
    }
}

impl WinsysEventPumpSdl2<'_>{
    fn handle_event(&self, event : sdl2::event::Event) {

        // handle sdl2 events

        use sdl2::event::Event;
        match event {
                Event::Quit { .. } => {
                        self.data_.lock().unwrap().done = true;
                        return;
                },
                Event::KeyUp { scancode, repeat, .. } => {
                    let sc = scancode.unwrap() as i32;
                    let eh = self.event_handler.write().unwrap();
                    if !repeat {
                        eh.as_ref().unwrap().broadcast_event(&self.keyboard_events_map[&sc][2]);
                        eh.as_ref().unwrap().derepeat_event(&self.keyboard_events_map[&sc][1]);
                    }
                    
                    //println!("{}", repeat);
                }
                Event::KeyDown { scancode, repeat, .. } => {
                    let sc = scancode.unwrap() as i32;
                    let eh = self.event_handler.write().unwrap();
                    if !repeat {
                        eh.as_ref().unwrap().broadcast_event(&self.keyboard_events_map[&sc][0]);
                        eh.as_ref().unwrap().repeat_event(&self.keyboard_events_map[&sc][1]);
                    }
                }
                Event::MouseButtonUp { mouse_btn, .. } => {
                    let sc = mouse_btn as i32;
                    let eh = self.event_handler.read().unwrap();
                    
                    eh.as_ref().unwrap().broadcast_event(&self.mouse_events_map[&sc][2]);
                    eh.as_ref().unwrap().derepeat_event(&self.mouse_events_map[&sc][1]);
                }
                Event::MouseButtonDown { mouse_btn, .. } => {
                    let sc = mouse_btn as i32;
                    let eh = self.event_handler.read().unwrap();

                    eh.as_ref().unwrap().broadcast_event(&self.mouse_events_map[&sc][0]);
                    eh.as_ref().unwrap().repeat_event(&self.mouse_events_map[&sc][1]);
                }
                Event::MouseMotion {x, y, xrel, yrel, ..} => {
                    let mut eh = self.event_handler.write().unwrap();
                    eh.as_mut().unwrap().update_mouse_status(MouseCoords{x, y}, MouseCoords{x:xrel, y:yrel});
                    let mm_id = eh.as_ref().unwrap().get_mouse_event_id("motion").unwrap();
                    eh.as_ref().unwrap().broadcast_event(&mm_id);
                }
                Event::Window { win_event, .. } => {
                    use sdl2::event::WindowEvent::*;
                    let mut resize =false;
                    let mut resize_dims = [0, 0];
                    match win_event{
                        Resized(x, y) => {
                            resize = true;
                            resize_dims = [x, y];
                        }
                        SizeChanged(x, y) => {
                            resize = true;
                            resize_dims = [x, y];
                        }
                        _ => {

                        }
                    }
                    if resize {
                        let mut data = self.data_.lock().unwrap();
                        data.update_info.res_x = u32::try_from(resize_dims[0]).unwrap();
                        data.update_info.res_y = u32::try_from(resize_dims[1]).unwrap();
                        data.update_info.res_changed = true;
                        unsafe {
                            gl::BindFramebuffer(gl::DRAW_BUFFER, 0);
                            gl::Viewport(0, 0, resize_dims[0], resize_dims[1]);
                        }
                    }
                }
                _ => {}
            }
    
    }
}

