fn main() -> wry::Result<()> {
    use tao::{
        event::{Event, StartCause, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        platform::unix::WindowExtUnix,
        window::WindowBuilder,
    };
    use wry::{WebViewBuilder, WebViewBuilderExtUnix};
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Notion - Your Digital Workspace")
        .build(&event_loop)
        .unwrap();
    let _webview = WebViewBuilder::new_gtk(window.gtk_window())
        .with_url("https://notion.so")?
        .build()?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => println!("Wry has started!"),
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}
