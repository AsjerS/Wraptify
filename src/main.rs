#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::Result;
use tao::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoopBuilder},
    window::{Theme, WindowBuilder},
};
use wry::WebViewBuilder;

#[derive(Debug, Clone, Copy)]
enum UserEvent {
    BootstrapReady,
}

const USER_AGENT: &str = "Mozilla/5.0 (X11; CrOS x86_64 14541.0.0) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36";
const SPOTIFY_URL: &str = "https://open.spotify.com/";
const BOOTSTRAP_HTML: &str = r#"
    <!DOCTYPE html>
    <html>
      <head>
        <style>body { background-color: #121212; }</style>
      </head>
      <body>
        <script>window.ipc.postMessage('bootstrapReady');</script>
      </body>
    </html>
"#;

fn main() -> Result<()> {
    let event_loop = EventLoopBuilder::<UserEvent>::with_user_event().build();

    let window = WindowBuilder::new()
        .with_title("Wraptify")
        .with_inner_size(LogicalSize::new(1200, 1000))
        .with_visible(false)
        .with_theme(Some(Theme::Dark))
        .build(&event_loop)?;

    let proxy = event_loop.create_proxy();

    let ipc_handler = move |request: wry::http::Request<String>| {
        if request.body() == "bootstrapReady" {
            let _ = proxy.send_event(UserEvent::BootstrapReady);
        }
    };

    let webview = WebViewBuilder::new()
        .with_html(BOOTSTRAP_HTML)
        .with_user_agent(USER_AGENT)
        .with_background_color((18, 18, 18, 255))
        .with_devtools(cfg!(debug_assertions))
        .with_ipc_handler(ipc_handler)
        .build(&window)?;

    let mut is_loaded = false;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::UserEvent(UserEvent::BootstrapReady) => {
                if !is_loaded {
                    window.set_visible(true);
                    let _ = webview.load_url(SPOTIFY_URL);
                    is_loaded = true;
                }
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
                ..
            } if window_id == window.id() => {
                *control_flow = ControlFlow::Exit;
            }
            _ => (),
        }
    });
}
