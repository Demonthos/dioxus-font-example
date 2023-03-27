#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_desktop::Config;

fn main() {
    dioxus_desktop::launch_cfg(
        app,
        Config::new().with_custom_head(
            r#"
            <style>
                @font-face {
                    font-family: CrotahFreeVersionItalic;
                    font-weight: normal;
                    src: url("./fonts/OpenDyslexic-Regular.otf");
                }
                h1 {
                    font-family: 'CrotahFreeVersionItalic';
                }
            </style>
            "#
            .into(),
        ),
    );
}

fn app(cx: Scope) -> Element {
    render! {
        h1 {
            "Hello, world!"
        }
    }
}
