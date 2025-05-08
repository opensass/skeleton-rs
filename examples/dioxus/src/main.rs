use dioxus::prelude::*;
use dioxus_logger::tracing;
use skeleton_rs::dioxus::{Skeleton, SkeletonGroup};
use skeleton_rs::{Animation, Theme, Variant};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styles.css");

fn main() {
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
    tracing::info!("starting app");
    launch(app);
}

fn app() -> Element {
    rsx! {
        document::Script { src: "https://kit.fontawesome.com/8f223ead6e.js" },
        document::Stylesheet { href: "https://unpkg.com/tailwindcss@2.2.19/dist/tailwind.min.css" },
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Examples {}
    }
}

#[component]
fn Example1() -> Element {
    rsx! {
        div {
            class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md",
            h2 { class: "text-xl font-bold mb-2", "Basic Skeleton" }
            pre {
                class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                r#"use dioxus::prelude::*;
use skeleton_rs::dioxus::*;

#[component]
fn Example1() -> Element {{
    rsx! {{
        Skeleton {{
            variant: Variant::Image,
            width: "300px",
            height: "200px",
            show: false
        }}
    }}
}}"#
            }
            Skeleton {
                variant: Variant::Image,
                width: "300px",
                height: "200px",
                show: false
            }
        }
    }
}

#[component]
fn Example2() -> Element {
    rsx! {
        div {
            class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md",
            h2 { class: "text-xl font-bold mb-2", "Wave Text" }
            pre {
                class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                r#"use dioxus::prelude::*;
use skeleton_rs::dioxus::*;

#[component]
fn Example2() -> Element {{
    rsx! {{
        Skeleton {{
            variant: Variant::Text,
            width: "100%",
            height: "1.5em",
            border_radius: "0",
            animation: Animation::Wave,
            show: false
        }}
    }}
}}"#
            }
            Skeleton {
                variant: Variant::Text,
                width: "100%",
                height: "1.5em",
                border_radius: "0",
                animation: Animation::Wave,
                show: false
            }
        }
    }
}

#[component]
fn Example3() -> Element {
    rsx! {
        div {
            class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md",
            h2 { class: "text-xl font-bold mb-2", "Dark Avatar" }
            pre {
                class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                r#"use dioxus::prelude::*;
use skeleton_rs::dioxus::*;

#[component]
fn Example3() -> Element {{
    rsx! {{
        Skeleton {{
            variant: Variant::Avatar,
            width: "80px",
            height: "80px",
            theme: Theme::Dark,
            animation: Animation::Pulse,
            show: false
        }}
    }}
}}"#
            }
            Skeleton {
                variant: Variant::Avatar,
                width: "80px",
                height: "80px",
                theme: Theme::Dark,
                animation: Animation::Pulse,
                show: false
            }
        }
    }
}

#[component]
fn Example4() -> Element {
    rsx! {
        div {
            class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md",
            h2 { class: "text-xl font-bold mb-2", "Button Skeleton" }
            pre {
                class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                r#"use dioxus::prelude::*;
use skeleton_rs::dioxus::*;

#[component]
fn Example4() -> Element {{
    rsx! {{
        Skeleton {{
            variant: Variant::Button,
            width: "150px",
            height: "40px",
            animate_on_hover: true,
            animate_on_active: true,
            animation: Animation::Pulse,
            theme: Theme::Custom("\#0099ff"),
            show: false
        }}
    }}
}}"#
            }
            Skeleton {
                variant: Variant::Button,
                width: "150px",
                height: "40px",
                animate_on_hover: true,
                animate_on_active: true,
                animation: Animation::Pulse,
                theme: Theme::Custom("#0099ff"),
                show: false
            }
        }
    }
}

#[component]
fn Example5() -> Element {
    rsx! {
        div {
            class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md",
            h2 { class: "text-xl font-bold mb-2", "Skeleton Group" }
            pre {
                class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                r#"use dioxus::prelude::*;
use skeleton_rs::dioxus::*;

#[component]
fn Example5() -> Element {{
    rsx! {{
        SkeletonGroup {{
            style: "display: flex; gap: 1rem;",
            Skeleton {{ variant: Variant::Rounded, width: "100px", height: "100px" }}
            Skeleton {{ variant: Variant::Rounded, width: "100px", height: "100px" }}
            Skeleton {{ variant: Variant::Rounded, width: "100px", height: "100px" }}
        }}
    }}
}}"#
            }
            SkeletonGroup {
                style: "display: flex; gap: 1rem;",
                Skeleton { variant: Variant::Rounded, width: "100px", height: "100px" }
                Skeleton { variant: Variant::Rounded, width: "100px", height: "100px" }
                Skeleton { variant: Variant::Rounded, width: "100px", height: "100px" }
            }
        }
    }
}

#[component]
fn Example6() -> Element {
    rsx! {
        div {
            class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md",
            h2 { class: "text-xl font-bold mb-2", "Animate on Visible" }
            pre {
                class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                r#"use dioxus::prelude::*;
use skeleton_rs::dioxus::*;

#[component]
fn Example6() -> Element {{
    rsx! {{
        Skeleton {{
            variant: Variant::Text,
            animation: Animation::Wave,
            animate_on_visible: true,
            width: "150px",
            height: "40px",
        }}
    }}
}}"#
            }
            div {
                id: "skeleton-rs",
                Skeleton {
                    variant: Variant::Text,
                    animation: Animation::Wave,
                    animate_on_visible: true,
                    width: "150px",
                    height: "40px",
                }
            }
        }
    }
}

#[component]
fn Example7() -> Element {
    rsx! {
        div {
            class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md",
            h2 { class: "text-xl font-bold mb-2", "Responsive Image Skeleton" }
            pre {
                class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                r#"use dioxus::prelude::*;
use skeleton_rs::dioxus::*;

#[component]
fn Example7() -> Element {{
    rsx! {{
        Skeleton {{
            variant: Variant::Image,
            width: "100%",
            height: "400px",
            responsive: true,
            animate_on_focus: true,
            animation: Animation::Wave
        }}
    }}
}}"#
            }
            Skeleton {
                variant: Variant::Image,
                width: "100%",
                height: "400px",
                responsive: true,
                animate_on_focus: true,
                animation: Animation::Wave
            }
        }
    }
}

#[component]
fn Example8() -> Element {
    rsx! {
        div {
            class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md",
            h2 { class: "text-xl font-bold mb-2", "Styled Button" }
            pre {
                class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                r#"use dioxus::prelude::*;
use skeleton_rs::dioxus::*;

#[component]
fn Example8() -> Element {{
    rsx! {{
        Skeleton {{
            variant: Variant::Button,
            width: "200px",
            height: "50px",
            animation: Animation::Pulse,
            custom_style: "box-shadow: 0 0 10px rgba(0,0,0,0.2);"
        }}
    }}
}}"#
            }
            Skeleton {
                variant: Variant::Button,
                width: "200px",
                height: "50px",
                animation: Animation::Pulse,
                custom_style: "box-shadow: 0 0 10px rgba(0,0,0,0.2);"
            }
        }
    }
}

#[component]
fn Example10() -> Element {
    rsx! {
        div {
            class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md",
            h2 { class: "text-xl font-bold mb-2", "Delayed Skeleton" }
            pre {
                class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                r#"use dioxus::prelude::*;
use skeleton_rs::dioxus::*;

#[component]
fn Example10() -> Element {{
    rsx! {{
        Skeleton {{
            variant: Variant::Rounded,
            width: "100%",
            height: "250px",
            delay_ms: 1000,
            show: false
        }}
    }}
}}"#
            }
            Skeleton {
                variant: Variant::Rounded,
                width: "100%",
                height: "250px",
                delay_ms: 1000,
                show: false
            }
        }
    }
}

#[component]
fn Example11() -> Element {
    rsx! {
        div {
            class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md",
            h2 { class: "text-xl font-bold mb-2", "Custom Style Skeleton" }
            pre {
                class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                r#"use dioxus::prelude::*;
use skeleton_rs::dioxus::*;

#[component]
fn Example11() -> Element {{
    rsx! {{
        Skeleton {{
            variant: Variant::Image,
            width: "100%",
            height: "300px",
            animation: Animation::Wave,
            min_height: "100px",
            max_height: "400px",
            custom_style: "border: 2px dashed red;"
        }}
    }}
}}"#
            }
            Skeleton {
                variant: Variant::Image,
                width: "100%",
                height: "300px",
                animation: Animation::Wave,
                min_height: "100px",
                max_height: "400px",
                custom_style: "border: 2px dashed red;"
            }
        }
    }
}

#[component]
fn Examples() -> Element {
    rsx! {
        div {
            class: "m-6 min-h-screen flex flex-col items-center justify-center",
            h1 { class: "text-3xl font-bold mb-8 text-white", "Skeleton RS Dioxus Examples" }
            div {
                class: "grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-8",
                Example1 {}
                Example2 {}
                Example3 {}
                Example4 {}
                Example5 {}
                Example6 {}
                Example7 {}
                Example8 {}
                Example10 {}
                Example11 {}
            }
        }
    }
}
