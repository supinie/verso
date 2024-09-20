// Prevent console window from appearing on Windows
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use verso::config::Config;
use verso::{Result, Verso};
use winit::application::ApplicationHandler;
use winit::event_loop::{self, DeviceEvents};
use winit::event_loop::{EventLoop, EventLoopProxy};

struct App {
    verso: Option<Verso>,
    proxy: EventLoopProxy<()>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let config = Config::new(resources_dir_path().unwrap());
        self.verso = Some(Verso::new(event_loop, self.proxy.clone(), config));
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        if let Some(v) = self.verso.as_mut() {
            v.handle_winit_window_event(window_id, event);
            v.handle_servo_messages(event_loop);
        }
    }

    fn user_event(&mut self, event_loop: &event_loop::ActiveEventLoop, _: ()) {
        if let Some(v) = self.verso.as_mut() {
            v.handle_servo_messages(event_loop);
        }
    }
}

fn main() -> Result<()> {
    let event_loop = EventLoop::new()?;
    event_loop.listen_device_events(DeviceEvents::Never);
    let proxy = event_loop.create_proxy();
    let mut app = App { verso: None, proxy };
    event_loop.run_app(&mut app)?;

    Ok(())
}

fn resources_dir_path() -> Option<std::path::PathBuf> {
    #[cfg(feature = "packager")]
    let root_dir = {
        use cargo_packager_resource_resolver::{current_format, resources_dir};
        current_format().and_then(|format| resources_dir(format))
    };
    #[cfg(feature = "flatpak")]
    let root_dir = {
        use std::str::FromStr;
        std::path::PathBuf::from_str("/app")
    };
    #[cfg(feature = "nixpkgs")]
    let root_dir = std::env::current_dir();
    #[cfg(not(any(feature = "packager", feature = "flatpak", feature = "nixpkgs")))]
    let root_dir = std::env::current_dir();

    let output = root_dir.ok().map(|dir| dir.join("resources"));

    println!("{output:?}");
    output
}
