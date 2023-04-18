use irisia::{
    application::Window,
    element::{Element, Frame, NeverInitalized, NoProps, RuntimeInit},
    event::standard::{
        Blured, Click, ElementAbondoned, ElementCreated, Focused, PointerEntered, PointerMove,
        PointerOut,
    },
    exit_app,
    primary::Point,
    read_style, render_fn,
    skia_safe::{Color, Color4f, Paint, Rect},
    structure::{StructureBuilder, VisitIter},
    style,
    style::StyleColor,
    textbox::{styles::*, TextBox},
    Event, StaticWindowEvent, Style,
};
use tokio::select;

#[irisia::main]
async fn main() {
    Window::new::<App>("test".into())
        .await
        .unwrap()
        .join()
        .await;
}

struct App {
    rects: Vec<Color>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            rects: vec![
                Color::RED,
                Color::YELLOW,
                Color::BLUE,
                Color::GREEN,
                Color::BLACK,
            ],
        }
    }
}

impl Element for App {
    type Props<'a> = NoProps;
    type ChildProps<'a> = NeverInitalized;

    render_fn! {
        @init(self);
        Flex {
            TextBox {
                text: "hello世界🌏",
                +id: "textbox",
                +style: style!{
                    color: Color::MAGENTA;
                    font_slant: .normal;
                    font_size: 50px;
                }
            }

            for (index, color) in self.rects.iter().enumerate() {
                @key index;
                Rectangle {
                    +id: ("rect", index),
                    +style: style!{
                        width: 100.0;
                        height: 100.0 + 40.0 * index as f32;
                        color: color.clone();
                    }
                }
            }
        }
    }

    fn start_runtime(init: RuntimeInit<Self>) {
        tokio::spawn(async move {
            let a = async {
                loop {
                    let ElementCreated { result, key } = init
                        .element_handle
                        .get_element_checked(|(s, _): &(&str, usize)| *s == "rect")
                        .await;

                    tokio::spawn(async move {
                        println!("recv{} got", key.1);

                        loop {
                            tokio::select! {
                                _ = result.recv_sys::<Focused>() => {
                                    println!("rectangle {} gained focus",key.1);
                                }
                                _ = result.recv_sys::<Blured>() => {
                                    println!("rectangle {} lost focus",key.1);
                                }
                                _ = result.recv_sys::<Click>() => {
                                    println!("rectangle {} clicked", key.1);
                                }
                                _ = result.recv::<MyRequestClose>() => {
                                    println!("close request event received(sent by {:?})", key.1);
                                    init.close_handle.close();
                                    break;
                                }
                            }
                        }
                    });
                }
            };

            let b = async {
                let ele = init.element_handle.get_element_eq(&"textbox").await;
                ele.recv_sys::<PointerMove>().await;
                tokio::spawn(async move {
                    loop {
                        ele.hover().await;
                        println!("cursor hovering on textbox");
                        ele.hover_canceled().await;
                        println!("cursor hovering canceled");
                    }
                });
            };

            tokio::join!(a, b);
        });
    }
}

#[derive(Style, Clone)]
#[irisia(from)]
struct StyleWidth(f32);

#[derive(Style, Clone)]
#[irisia(from)]
struct StyleHeight(f32);

struct Rectangle {
    is_force: bool,
    force_color: Color,
}

impl Default for Rectangle {
    fn default() -> Self {
        Self {
            is_force: false,
            force_color: Color::CYAN,
        }
    }
}

impl Element for Rectangle {
    type Props<'a> = NoProps;
    type ChildProps<'a> = ();

    fn render<'a>(
        &mut self,
        Frame {
            styles,
            drawing_region: region,
            mut content,
            ..
        }: irisia::element::Frame<
            Self,
            impl style::StyleContainer,
            impl VisitIter<Self::ChildProps<'a>>,
        >,
    ) -> irisia::Result<()> {
        read_style!(styles => {
            w: Option<StyleWidth>,
            h: Option<StyleHeight>,
            c: Option<StyleColor>,
        });

        let (w, h) = (
            w.unwrap_or(StyleWidth(50.0)),
            h.unwrap_or(StyleHeight(50.0)),
        );

        content.set_interact_region((region.0, region.0 + Point(w.0 as _, h.0 as _)));

        let rect = Rect::new(
            region.0 .0 as _,
            region.0 .1 as _,
            region.0 .0 as f32 + w.0,
            region.0 .1 as f32 + h.0,
        );
        let color = if self.is_force {
            self.force_color.clone()
        } else {
            c.unwrap_or(StyleColor(Color::GREEN)).0
        };

        let paint = Paint::new(Color4f::from(color), None);

        content.canvas().draw_rect(rect, &paint);

        Ok(())
    }

    fn start_runtime(init: RuntimeInit<Self>) {
        tokio::spawn(async move {
            let a = async {
                loop {
                    let window_event = init
                        .window_event_dispatcher
                        .recv_sys::<StaticWindowEvent>()
                        .await;
                    match window_event {
                        StaticWindowEvent::CloseRequested => {
                            println!("close event sent");
                            init.element_handle.emit(MyRequestClose);
                        }

                        _ => {}
                    }
                }
            };

            let b = async {
                loop {
                    select! {
                        _ = init.recv_sys::<ElementAbondoned>() => {
                            println!("element dropped");
                            exit_app(0).await;
                            return;
                        },

                        _=init.recv_sys::<PointerEntered>()=>{
                            init.app.lock().await.is_force=true;
                        }

                        _=init.recv_sys::<PointerOut>()=>{
                            init.app.lock().await.is_force=false;
                        }
                    }
                }
            };

            let c = async {
                loop {
                    select! {
                        _ = init.recv_sys::<PointerEntered>() => {
                            println!("pointer entered");
                        },

                        _ = init.recv_sys::<PointerOut>() => {
                            println!("pointer out");
                        },
                    }
                }
            };

            tokio::join!(a, b, c);
        });
    }
}

#[derive(Event, Clone)]
pub struct MyRequestClose;

#[derive(Default)]
struct Flex;

impl Element for Flex {
    type Props<'a> = NoProps;
    type ChildProps<'a> = ();

    fn render<'a>(
        &mut self,
        Frame {
            drawing_region,
            mut content,
            children,
            ..
        }: Frame<Self, impl style::StyleContainer, impl VisitIter<Self::ChildProps<'a>>>,
    ) -> irisia::Result<()> {
        let (start, end) = drawing_region;
        let abs = end - start;

        let rendering = children.into_rendering(&mut content);
        let len = rendering.children_count();
        let width = abs.0 / len as u32;

        let mut index = 0;
        rendering.finish_iter(|(), _| {
            let result = Ok((
                Point(index * width, start.1),
                Point((index + 1) * width, end.1),
            ));
            index += 1;
            result
        })
    }
}
