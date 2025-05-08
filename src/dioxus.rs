#![doc = include_str!("../DIOXUS.md")]

use crate::common::{Animation, Direction, Theme, Variant};
use dioxus::prelude::*;
use gloo_timers::callback::Timeout;
use web_sys::js_sys;
use web_sys::wasm_bindgen::JsCast;
use web_sys::wasm_bindgen::prelude::*;
use web_sys::window;
use web_sys::{IntersectionObserver, IntersectionObserverEntry};

/// Properties for the `Skeleton` component.
#[derive(Props, PartialEq, Clone)]
pub struct SkeletonProps {
    /// Child elements to render inside the skeleton.
    ///
    /// If provided, the children will be wrapped with the skeleton styling and animation.
    #[props(default)]
    pub children: Element,

    /// The visual variant of the skeleton.
    ///
    /// Variants control the shape or type of the skeleton placeholder, such as text or circle.
    /// Defaults to `Variant::Text`.
    #[props(default)]
    pub variant: Variant,

    /// Animation style applied to the skeleton.
    ///
    /// Controls how the skeleton animates, e.g., pulse, wave, etc.
    /// Defaults to `Animation::Pulse`.
    #[props(default)]
    pub animation: Animation,

    /// Direction of the animation direction and background color gradient.
    #[props(default)]
    pub direction: Direction,

    /// The theme of the skeleton appearance.
    ///
    /// Allows switching between light or dark themes.
    /// Defaults to `Theme::Light`.
    #[props(default)]
    pub theme: Theme,

    /// The width of the skeleton.
    ///
    /// Accepts any valid CSS width value (e.g., `100%`, `200px`, `10rem`). Defaults to `"100%"`.
    #[props(default = "100%")]
    pub width: &'static str,

    /// The height of the skeleton.
    ///
    /// Accepts any valid CSS height value. Defaults to `"1em"`.
    #[props(default = "1em")]
    pub height: &'static str,

    /// Optional font size for the skeleton text.
    ///
    /// Used to size the placeholder in proportion to text elements. If not set, font size is not applied.
    #[props(default)]
    pub font_size: Option<&'static str>,

    /// Border radius for the skeleton.
    ///
    /// Controls the rounding of the skeleton's corners. Accepts any valid CSS radius.
    /// Defaults to `"4px"`.
    #[props(default = "4px")]
    pub border_radius: &'static str,

    /// Display property for the skeleton.
    ///
    /// Determines the skeleton's display type (e.g., `inline-block`, `block`). Defaults to `"inline-block"`.
    #[props(default = "inline-block")]
    pub display: &'static str,

    /// Line height of the skeleton content.
    ///
    /// This affects vertical spacing in text-like skeletons. Defaults to `"1"`.
    #[props(default = "1")]
    pub line_height: &'static str,

    /// The CSS `position` property.
    ///
    /// Controls how the skeleton is positioned. Defaults to `"relative"`.
    #[props(default = "relative")]
    pub position: &'static str,

    /// Overflow behavior of the skeleton container.
    ///
    /// Accepts values like `hidden`, `visible`, etc. Defaults to `"hidden"`.
    #[props(default = "hidden")]
    pub overflow: &'static str,

    /// Margin applied to the skeleton.
    ///
    /// Accepts any valid CSS margin value. Defaults to `""`.
    #[props(default)]
    pub margin: &'static str,

    /// Additional inline styles.
    ///
    /// Allows you to append arbitrary CSS to the skeleton component. Useful for quick overrides.
    #[props(default)]
    pub custom_style: &'static str,

    /// Whether to automatically infer the size from children.
    ///
    /// If `true`, the skeleton will try to match the dimensions of its content.
    #[props(default)]
    pub infer_size: bool,

    /// Whether the skeleton is currently visible.
    ///
    /// Controls whether the skeleton should be rendered or hidden.
    #[props(default)]
    pub show: bool,

    /// Delay before the skeleton becomes visible, in milliseconds.
    ///
    /// Useful for preventing flicker on fast-loading content. Defaults to `0`.
    #[props(default = 0)]
    pub delay_ms: u32,

    /// Whether the skeleton is responsive.
    ///
    /// Enables responsive resizing behavior based on the parent container or screen size.
    #[props(default)]
    pub responsive: bool,

    /// Optional maximum width of the skeleton.
    ///
    /// Accepts any valid CSS width value (e.g., `600px`, `100%`).
    #[props(default)]
    pub max_width: Option<&'static str>,

    /// Optional minimum width of the skeleton.
    ///
    /// Accepts any valid CSS width value.
    #[props(default)]
    pub min_width: Option<&'static str>,

    /// Optional maximum height of the skeleton.
    ///
    /// Accepts any valid CSS height value.
    #[props(default)]
    pub max_height: Option<&'static str>,

    /// Optional minimum height of the skeleton.
    ///
    /// Accepts any valid CSS height value.
    #[props(default)]
    pub min_height: Option<&'static str>,

    /// Whether the skeleton animates on hover.
    ///
    /// When enabled, an animation will be triggered when the user hovers over the skeleton.
    #[props(default)]
    pub animate_on_hover: bool,

    /// Whether the skeleton animates on focus.
    ///
    /// Useful for accessibility - triggers animation when the component receives focus.
    #[props(default)]
    pub animate_on_focus: bool,

    /// Whether the skeleton animates on active (click or tap).
    ///
    /// Triggers animation when the skeleton is actively clicked or touched.
    #[props(default)]
    pub animate_on_active: bool,

    /// Whether the skeleton animates when it becomes visible in the viewport.
    ///
    /// Uses `IntersectionObserver` to detect visibility and trigger animation.
    #[props(default)]
    pub animate_on_visible: bool,
}

/// Skeleton Component
///
/// A flexible and customizable `Skeleton` component for Dioxus applications, ideal for
/// rendering placeholder content during loading states. It supports
/// animations, visibility-based rendering, and responsive behavior.
///
/// # Properties
/// The component uses the `SkeletonProps` struct for its properties. Key properties include:
///
/// # Features
/// - **Viewport-aware Animation**: When `animate_on_visible` is enabled, the component uses `IntersectionObserver` to trigger animation only when the element is scrolled into view.
///
/// - **Delay Support**: Prevents immediate rendering using the `delay_ms` prop, useful for avoiding flicker on fast-loading content.
///
/// - **Responsive Layout**: With the `responsive` prop, skeletons scale naturally across screen sizes.
///
/// - **State-controlled Rendering**: You can explicitly show or hide the skeleton using the `show` prop or control visibility dynamically.
///
/// - **Slot Support**:
///   You can pass children to be wrapped in the skeleton effect, especially useful for text or dynamic UI blocks.
///
/// # Examples
///
/// ## Basic Usage
/// ```rust
/// use dioxus::prelude::*;
/// use skeleton_rs::dioxus::Skeleton;
///
/// fn App() -> Element {
///     rsx! {
///         Skeleton {
///             width: "200px",
///             height: "1.5em"
///         }
///     }
/// }
/// ```
///
/// ## Text Placeholder
/// ```rust
/// use dioxus::prelude::*;
/// use skeleton_rs::dioxus::Skeleton;
/// use skeleton_rs::Variant;
///
/// fn App() -> Element {
///     rsx! {
///         Skeleton {
///             variant: Variant::Text,
///             width: "100%",
///             height: "1.2em"
///         }
///     }
/// }
/// ```
///
/// ## Responsive with Inferred Size
/// ```rust
/// use dioxus::prelude::*;
/// use skeleton_rs::dioxus::Skeleton;
///
/// fn App() -> Element {
///     rsx! {
///         Skeleton {
///             infer_size: true,
///             responsive: true,
///             p { "Loading text..." }
///         }
///     }
/// }
/// ```
///
/// ## Animate When Visible
/// ```rust
/// use dioxus::prelude::*;
/// use skeleton_rs::dioxus::Skeleton;
/// use skeleton_rs::Variant;
///
/// fn App() -> Element {
///     rsx! {
///         Skeleton {
///             variant: Variant::Text,
///             animate_on_visible: true,
///             width: "80%",
///             height: "2em"
///         }
///     }
/// }
/// ```
///
/// # Behavior
/// - With `animate_on_visible`, the animation begins only when the skeleton is in the viewport.
/// - When `show` is false, the component stays hidden until external or internal logic reveals it.
/// - Most style attributes can be customized via props.
///
/// # Accessibility
/// - Skeletons typically serve as non-interactive placeholders and are not announced by screen readers.
/// - For better accessibility, use parent-level ARIA attributes like `aria-busy`, `aria-hidden`, or live regions.
///
/// # Notes
/// - The component uses `NodeRef` internally to track visibility using `IntersectionObserver`.
/// - Child content provided via the `children` prop is rendered and masked by the skeleton effect.
///
/// # See Also
/// - [MDN IntersectionObserver](https://developer.mozilla.org/en-US/docs/Web/API/Intersection_Observer_API)
#[component]
pub fn Skeleton(props: SkeletonProps) -> Element {
    let mut visible = use_signal(|| !props.show);
    let id = "skeleton-rs";

    use_effect(move || {
        if props.show {
            visible.set(false);
        } else if props.delay_ms > 0 {
            Timeout::new(props.delay_ms, move || {
                visible.set(true);
            })
            .forget();
        } else {
            visible.set(true);
        }
    });

    if props.animate_on_visible {
        use_effect(move || {
            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();
            if let Some(element) = document.get_element_by_id(id) {
                let closure = Closure::wrap(Box::new(
                    move |entries: js_sys::Array, _obs: IntersectionObserver| {
                        for entry in entries.iter() {
                            let entry: IntersectionObserverEntry = entry.unchecked_into();
                            if entry.is_intersecting() {
                                visible.set(true);
                            }
                        }
                    },
                )
                    as Box<dyn FnMut(js_sys::Array, IntersectionObserver)>);

                let observer = IntersectionObserver::new(closure.as_ref().unchecked_ref()).unwrap();
                observer.observe(&element);
                closure.forget();
            }
        });
    }

    let background_color = match props.theme {
        Theme::Light => "#e0e0e0",
        Theme::Dark => "#444444",
        Theme::Custom(color) => color,
    };

    let effective_radius = match props.variant {
        Variant::Circular | Variant::Avatar => "50%",
        Variant::Rectangular => "0",
        Variant::Rounded => "8px",
        Variant::Button => "6px",
        Variant::Text | Variant::Image => props.border_radius,
    };

    let animation_style = match props.animation {
        Animation::Pulse => "animation: skeleton-rs-pulse 1.5s ease-in-out infinite;".to_string(),
        Animation::Wave => {
            let angle = match props.direction {
                Direction::LeftToRight => 90,
                Direction::RightToLeft => 270,
                Direction::TopToBottom => 180,
                Direction::BottomToTop => 0,
                Direction::CustomAngle(deg) => deg,
            };

            format!(
                "background: linear-gradient({}deg, #e0e0e0 25%, #f5f5f5 50%, #e0e0e0 75%);
                 background-size: 200% 100%;
                 animation: skeleton-rs-wave 1.6s linear infinite;",
                angle
            )
        }
        Animation::None => "".to_string(),
    };

    let mut style = String::new();
    if props.infer_size {
        style.push_str(&format!(
            "background-color: {background_color}; border-radius: {effective_radius}; display: {}; position: {}; overflow: {}; margin: {};",
            props.display, props.position, props.overflow, props.margin
        ));
    } else {
        style.push_str(&format!(
            "width: {}; height: {}; background-color: {background_color}; border-radius: {effective_radius}; display: {}; position: {}; overflow: {}; margin: {}; line-height: {};",
            props.width, props.height, props.display, props.position, props.overflow, props.margin, props.line_height
        ));
    }

    if let Some(size) = props.font_size {
        style.push_str(&format!(" font-size: {size};"));
    }
    if let Some(max_w) = props.max_width {
        style.push_str(&format!(" max-width: {max_w};"));
    }
    if let Some(min_w) = props.min_width {
        style.push_str(&format!(" min-width: {min_w};"));
    }
    if let Some(max_h) = props.max_height {
        style.push_str(&format!(" max-height: {max_h};"));
    }
    if let Some(min_h) = props.min_height {
        style.push_str(&format!(" min-height: {min_h};"));
    }

    style.push_str(&animation_style);
    style.push_str(props.custom_style);

    let mut class_names = "skeleton-rs".to_string();
    if props.animate_on_hover {
        class_names.push_str(" skeleton-hover");
    }
    if props.animate_on_focus {
        class_names.push_str(" skeleton-focus");
    }
    if props.animate_on_active {
        class_names.push_str(" skeleton-active");
    }

    let direction = props.direction.clone();
    use_effect(move || {
        let window = window().unwrap();
        let document = window.document().unwrap();
        if document.get_element_by_id("skeleton-rs-style").is_none() {
            let style_elem = document.create_element("style").unwrap();
            style_elem.set_id("skeleton-rs-style");

            let wave_keyframes = match direction {
                Direction::LeftToRight => {
                    r#"
                        @keyframes skeleton-rs-wave {
                            0%   { background-position: 200% 0; }
                            25%  { background-position: 100% 0; }
                            50%  { background-position: 0% 0; }
                            75%  { background-position: -100% 0; }
                            100% { background-position: -200% 0; }
                        }"#
                }
                Direction::RightToLeft => {
                    r#"
                        @keyframes skeleton-rs-wave {
                            0%   { background-position: -200% 0; }
                            25%  { background-position: -100% 0; }
                            50%  { background-position: 0% 0; }
                            75%  { background-position: 100% 0; }
                            100% { background-position: 200% 0; }
                        }"#
                }
                Direction::TopToBottom => {
                    r#"
                        @keyframes skeleton-rs-wave {
                            0%   { background-position: 0 -200%; }
                            25%  { background-position: 0 -100%; }
                            50%  { background-position: 0 0%; }
                            75%  { background-position: 0 100%; }
                            100% { background-position: 0 200%; }
                        }"#
                }
                Direction::BottomToTop => {
                    r#"
                        @keyframes skeleton-rs-wave {
                            0%   { background-position: 0 200%; }
                            25%  { background-position: 0 100%; }
                            50%  { background-position: 0 0%; }
                            75%  { background-position: 0 -100%; }
                            100% { background-position: 0 -200%; }
                        }"#
                }
                Direction::CustomAngle(_) => {
                    r#"
                        @keyframes skeleton-rs-wave {
                            0%   { background-position: 200% 0; }
                            25%  { background-position: 100% 0; }
                            50%  { background-position: 0% 0; }
                            75%  { background-position: -100% 0; }
                            100% { background-position: -200% 0; }
                        }"#
                }
            };

            let css = format!(
                r#"
                        @keyframes skeleton-rs-pulse {{
                            0% {{ opacity: 1; }}
                            25% {{ opacity: 0.7; }}
                            50% {{ opacity: 0.4; }}
                            75% {{ opacity: 0.7; }}
                            100% {{ opacity: 1; }}
                        }}

                        {}

                        .skeleton-hover:hover {{
                            filter: brightness(0.95);
                        }}

                        .skeleton-focus:focus {{
                            outline: 2px solid #999;
                        }}

                        .skeleton-active:active {{
                            transform: scale(0.98);
                        }}
                    "#,
                wave_keyframes
            );

            style_elem.set_inner_html(&css);
            if let Some(head) = document.head() {
                head.append_child(&style_elem).unwrap();
            }
        }
    });

    if visible() {
        rsx! {
            div {
                id: "{id}",
                class: "{class_names}",
                style: "{style}",
                role: "presentation",
                aria_hidden: "true"
            }
        }
    } else {
        rsx! {
            {props.children}
        }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct SkeletonGroupProps {
    #[props(default)]
    pub children: Element,

    #[props(default)]
    pub style: &'static str,

    #[props(default)]
    pub class: &'static str,
}

#[component]
pub fn SkeletonGroup(props: SkeletonGroupProps) -> Element {
    rsx! {
        div {
            class: "{props.class}",
            style: "{props.style}",
            {props.children}
        }
    }
}
