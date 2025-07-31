#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::Result;
use tao::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use wry::WebViewBuilder;

fn main() -> Result<()> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Wraptify")
        .with_inner_size(LogicalSize::new(1200, 1000))
        .build(&event_loop)?;

    // prevents "download our app pls" ads
    let user_agent = "Mozilla/5.0 (X11; CrOS x86_64 14541.0.0) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36";

    let _webview = WebViewBuilder::new()
        .with_url("https://open.spotify.com/")
        .with_user_agent(user_agent)
        .with_devtools(cfg!(debug_assertions))
        .build(&window)?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            window_id,
            ..
        } = event
        {
            if window_id == window.id() {
                *control_flow = ControlFlow::Exit;
            }
        }
    });
}