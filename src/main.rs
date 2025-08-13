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

fn main() -> Result<()> {
    let event_loop = EventLoopBuilder::<UserEvent>::with_user_event().build();

    let window = WindowBuilder::new()
        .with_title("Wraptify")
        .with_inner_size(LogicalSize::new(1200, 1000))
        .with_visible(false)
        .with_theme(Some(Theme::Dark))
        .build(&event_loop)?;

    let user_agent = "Mozilla/5.0 (X11; CrOS x86_64 14541.0.0) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36";

    let bootstrap_html = r#"
        <!DOCTYPE html><html><head><style>body{background-color:#121212;}</style></head>
        <body><script>window.ipc.postMessage('bootstrapReady');</script></body></html>
    "#;

    let initialization_script = r#"
        (function() {
            function removeThirdPartyIframes() {
                const iframes = document.querySelectorAll('iframe');
                for (const iframe of iframes) {
                    try {
                        const url = new URL(iframe.src);
                        if (url.hostname !== 'open.spotify.com' && !url.hostname.endsWith('.spotify.com')) {
                            iframe.remove();
                        }
                    } catch (e) {
                        iframe.remove();
                    }
                }
            }
            setInterval(removeThirdPartyIframes, 2000);
        })();
    "#;

    let proxy = event_loop.create_proxy();
    let ipc_handler = move |request: wry::http::Request<String>| {
        if request.body() == "bootstrapReady" {
            proxy.send_event(UserEvent::BootstrapReady).unwrap();
        }
    };

    let webview = WebViewBuilder::new()
        .with_html(bootstrap_html)
        .with_user_agent(user_agent)
        .with_background_color((18, 18, 18, 255))
        .with_initialization_script(initialization_script)
        .with_devtools(cfg!(debug_assertions))
        .with_ipc_handler(ipc_handler)
        .build(&window)?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::UserEvent(UserEvent::BootstrapReady) => {
                window.set_visible(true);
                let _ = webview.load_url("https://open.spotify.com/");
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
