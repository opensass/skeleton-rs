#![doc = include_str!("../YEW.md")]

use crate::common::{Animation, Theme, Variant};
use gloo_timers::callback::Timeout;
use web_sys::js_sys;
use web_sys::wasm_bindgen::JsCast;
use web_sys::wasm_bindgen::prelude::*;
use web_sys::window;
use web_sys::{HtmlElement, IntersectionObserver, IntersectionObserverEntry};
use yew::prelude::*;


/// Properties for the `Skeleton` component.
#[derive(Properties, PartialEq, Clone)]
pub struct SkeletonProps {
    /// Child elements to render inside the skeleton.
    ///
    /// If provided, the children will be wrapped with the skeleton styling and animation.
    #[prop_or_default]
    pub children: Children,

    /// The visual variant of the skeleton.
    ///
    /// Variants control the shape or type of the skeleton placeholder, such as text or circle.
    /// Defaults to `Variant::Text`.
    #[prop_or_default]
    pub variant: Variant,

    /// Animation style applied to the skeleton.
    ///
    /// Controls how the skeleton animates, e.g., pulse, wave, etc.
    /// Defaults to `Animation::Pulse`.
    #[prop_or_default]
    pub animation: Animation,

    /// The theme of the skeleton appearance.
    ///
    /// Allows switching between light or dark themes.
    /// Defaults to `Theme::Light`.
    #[prop_or_default]
    pub theme: Theme,

    /// The width of the skeleton.
    ///
    /// Accepts any valid CSS width value (e.g., `100%`, `200px`, `10rem`). Defaults to `"100%"`.
    #[prop_or("100%")]
    pub width: &'static str,

    /// The height of the skeleton.
    ///
    /// Accepts any valid CSS height value. Defaults to `"1em"`.
    #[prop_or("1em")]
    pub height: &'static str,

    /// Optional font size for the skeleton text.
    ///
    /// Used to size the placeholder in proportion to text elements. If not set, font size is not applied.
    #[prop_or(None)]
    pub font_size: Option<&'static str>,

    /// Border radius for the skeleton.
    ///
    /// Controls the rounding of the skeleton's corners. Accepts any valid CSS radius.
    /// Defaults to `"4px"`.
    #[prop_or("4px")]
    pub border_radius: &'static str,

    /// Display property for the skeleton.
    ///
    /// Determines the skeleton's display type (e.g., `inline-block`, `block`). Defaults to `"inline-block"`.
    #[prop_or("inline-block")]
    pub display: &'static str,

    /// Line height of the skeleton content.
    ///
    /// This affects vertical spacing in text-like skeletons. Defaults to `"1"`.
    #[prop_or("1")]
    pub line_height: &'static str,

    /// The CSS `position` property.
    ///
    /// Controls how the skeleton is positioned. Defaults to `"relative"`.
    #[prop_or("relative")]
    pub position: &'static str,

    /// Overflow behavior of the skeleton container.
    ///
    /// Accepts values like `hidden`, `visible`, etc. Defaults to `"hidden"`.
    #[prop_or("hidden")]
    pub overflow: &'static str,

    /// Margin applied to the skeleton.
    ///
    /// Accepts any valid CSS margin value. Defaults to `""`.
    #[prop_or_default]
    pub margin: &'static str,

    /// Additional inline styles.
    ///
    /// Allows you to append arbitrary CSS to the skeleton component. Useful for quick overrides.
    #[prop_or_default]
    pub custom_style: &'static str,

    /// Whether to automatically infer the size from children.
    ///
    /// If `true`, the skeleton will try to match the dimensions of its content.
    #[prop_or(false)]
    pub infer_size: bool,

    /// Whether the skeleton is currently visible.
    ///
    /// Controls whether the skeleton should be rendered or hidden.
    #[prop_or(false)]
    pub show: bool,

    /// Delay before the skeleton becomes visible, in milliseconds.
    ///
    /// Useful for preventing flicker on fast-loading content. Defaults to `0`.
    #[prop_or(0)]
    pub delay_ms: u32,

    /// Whether the skeleton is responsive.
    ///
    /// Enables responsive resizing behavior based on the parent container or screen size.
    #[prop_or(false)]
    pub responsive: bool,

    /// Optional maximum width of the skeleton.
    ///
    /// Accepts any valid CSS width value (e.g., `600px`, `100%`).
    #[prop_or(None)]
    pub max_width: Option<&'static str>,

    /// Optional minimum width of the skeleton.
    ///
    /// Accepts any valid CSS width value.
    #[prop_or(None)]
    pub min_width: Option<&'static str>,

    /// Optional maximum height of the skeleton.
    ///
    /// Accepts any valid CSS height value.
    #[prop_or(None)]
    pub max_height: Option<&'static str>,

    /// Optional minimum height of the skeleton.
    ///
    /// Accepts any valid CSS height value.
    #[prop_or(None)]
    pub min_height: Option<&'static str>,

    /// Whether the skeleton animates on hover.
    ///
    /// When enabled, an animation will be triggered when the user hovers over the skeleton.
    #[prop_or(false)]
    pub animate_on_hover: bool,

    /// Whether the skeleton animates on focus.
    ///
    /// Useful for accessibility - triggers animation when the component receives focus.
    #[prop_or(false)]
    pub animate_on_focus: bool,

    /// Whether the skeleton animates on active (click or tap).
    ///
    /// Triggers animation when the skeleton is actively clicked or touched.
    #[prop_or(false)]
    pub animate_on_active: bool,

    /// Whether the skeleton animates when it becomes visible in the viewport.
    ///
    /// Uses `IntersectionObserver` to detect visibility and trigger animation.
    #[prop_or(false)]
    pub animate_on_visible: bool,
}

/// Skeleton Component
///
/// A flexible and customizable `Skeleton` component for Yew applications, ideal for
/// rendering placeholder content during loading states. It provides support for
/// animations, visibility-based rendering, and responsive behavior.
///
/// # Properties
/// The component uses the `SkeletonProps` struct for its properties. Key properties include:
///
/// # Features
/// - **Viewport-aware Animation**: When `animate_on_visible` is enabled, the component uses `IntersectionObserver` to trigger animation only when scrolled into view.
///
/// - **Delay Support**: Prevents immediate rendering using the `delay_ms` prop, useful for avoiding flash of skeletons for fast-loading content.
///
/// - **Responsive Layout**: With the `responsive` prop, skeletons scale naturally across screen sizes.
///
/// - **State-controlled Rendering**: You can explicitly show or hide the skeleton using the `show` prop or control visibility dynamically.
///
/// - **Slot Support**:
///   You can pass children to be wrapped in the skeleton effect, especially useful for text or UI blocks.
///
/// # Examples
///
/// ## Basic Usage
/// ```rust
/// use yew::prelude::*;
/// use skeleton_rs::yew::{Skeleton, SkeletonGroup};
/// use skeleton_rs::{Animation, Theme, Variant};
///
/// #[function_component(App)]
/// pub fn app() -> Html {
///     html! {
///         <Skeleton width="200px" height="1.5em" />
///     }
/// }
/// ```
///
/// ## Text Placeholder
/// ```rust
/// use yew::prelude::*;
/// use skeleton_rs::yew::{Skeleton, SkeletonGroup};
/// use skeleton_rs::{Animation, Theme, Variant};
///
/// #[function_component(App)]
/// pub fn app() -> Html {
///     html! {
///         <Skeleton variant={Variant::Text} width="100%" height="1.2em" />
///     }
/// }
/// ```
///
/// ## Responsive with Inferred Size
/// ```rust
/// use yew::prelude::*;
/// use skeleton_rs::yew::{Skeleton, SkeletonGroup};
/// use skeleton_rs::{Animation, Theme, Variant};
///
/// #[function_component(App)]
/// pub fn app() -> Html {
///     html! {
///         <Skeleton infer_size={true} responsive={true}>
///             <p>{ "Loading text..." }</p>
///         </Skeleton>
///     }
/// }
/// ```
///
/// ## Animate When Visible
/// ```rust
/// use yew::prelude::*;
/// use skeleton_rs::yew::{Skeleton, SkeletonGroup};
/// use skeleton_rs::{Animation, Theme, Variant};
///
/// #[function_component(App)]
/// pub fn app() -> Html {
///     html! {
///         <Skeleton
///             variant={Variant::Text}
///             animate_on_visible={true}
///             height="2em"
///             width="80%"
///         />
///     }
/// }
/// ```
///
/// # Behavior
/// - When `animate_on_visible` is enabled, animation starts only once the component enters the viewport.
/// - If `show` is set to `false`, the component initializes hidden and reveals itself based on internal or external logic.
/// - You can customize almost all styles using props.
///
/// # Accessibility
/// - Skeletons typically represent non-interactive placeholders and do not interfere with screen readers.
/// - Consider pairing them with appropriate ARIA `aria-busy`, `aria-hidden`, or live regions on the parent container for accessibility.
///
/// # Notes
/// - The component uses `NodeRef` internally to observe visibility changes.
/// - The `children` prop allows rendering actual elements inside the skeleton, which get masked by the animation.
///
/// # See Also
/// - [MDN IntersectionObserver](https://developer.mozilla.org/en-US/docs/Web/API/Intersection_Observer_API)
#[function_component(Skeleton)]
pub fn skeleton(props: &SkeletonProps) -> Html {
    let node_ref = use_node_ref();
    let visible = use_state(|| !props.show);

    let props_clone = props.clone();
    let visible_clone = visible.clone();

    {
        let visible = visible.clone();
        use_effect_with((props_clone.show,), move |_| {
            if props_clone.show {
                visible.set(false);
            } else if props_clone.delay_ms > 0 {
                let timeout = Timeout::new(props_clone.delay_ms, move || {
                    visible_clone.set(true);
                });
                timeout.forget();
            } else {
                visible.set(true);
            }
            || ()
        });
    }

    {
        let node_ref = node_ref.clone();
        let visible = visible.clone();

        use_effect_with(
            (node_ref.clone(), props.animate_on_visible),
            move |(node_ref, animate_on_visible)| {
                if !*animate_on_visible {
                    return;
                }

                let element = node_ref.cast::<HtmlElement>();
                if let Some(element) = element {
                    let cb = Closure::wrap(Box::new(
                        move |entries: js_sys::Array, _observer: IntersectionObserver| {
                            for entry in entries.iter() {
                                let entry = entry.unchecked_into::<IntersectionObserverEntry>();
                                if entry.is_intersecting() {
                                    visible.set(true);
                                }
                            }
                        },
                    )
                        as Box<dyn FnMut(js_sys::Array, IntersectionObserver)>);

                    let observer = IntersectionObserver::new(cb.as_ref().unchecked_ref()).unwrap();
                    observer.observe(&element);

                    cb.forget();
                }
            },
        );
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

    let base_animation = match props.animation {
        Animation::Pulse => "animation: skeleton-rs-pulse 1.5s ease-in-out infinite;",
        Animation::Wave => {
            "background: linear-gradient(90deg, #e0e0e0 25%, #f5f5f5 50%, #e0e0e0 75%); background-size: 200% 100%; animation: skeleton-rs-wave 1.6s linear infinite;"
        }
        Animation::None => "",
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

    style.push_str(base_animation);
    style.push_str(props.custom_style);

    let mut class_names = String::from("skeleton-rs");
    if props.animate_on_hover {
        class_names.push_str(" skeleton-hover");
    }
    if props.animate_on_focus {
        class_names.push_str(" skeleton-focus");
    }
    if props.animate_on_active {
        class_names.push_str(" skeleton-active");
    }

    use_effect_with((), move |_| {
        if let Some(doc) = window().and_then(|w| w.document()) {
            if doc.get_element_by_id("skeleton-rs-style").is_none() {
                let style_elem = doc.create_element("style").unwrap();
                style_elem.set_id("skeleton-rs-style");
                style_elem.set_inner_html(
                    r#"
                    @keyframes skeleton-rs-pulse {
                        0% { opacity: 1; }
                        50% { opacity: 0.4; }
                        100% { opacity: 1; }
                    }
                    @keyframes skeleton-rs-wave {
                        0% { background-position: -200% 0; }
                        100% { background-position: 200% 0; }
                    }
                    .skeleton-hover:hover {
                        filter: brightness(0.95);
                    }
                    .skeleton-focus:focus {
                        outline: 2px solid #999;
                    }
                    .skeleton-active:active {
                        transform: scale(0.98);
                    }
                "#,
                );
                if let Some(head) = doc.head() {
                    head.append_child(&style_elem).unwrap();
                }
            }
        }
    });

    if *visible {
        html! {
            <div
                ref={node_ref}
                class={class_names}
                style={style}
                role="presentation"
                aria-hidden="true"
            />
        }
    } else {
        html! { <>{ for props.children.iter() }</> }
    }
}

#[derive(Properties, PartialEq)]
pub struct SkeletonGroupProps {
    #[prop_or_default]
    pub children: ChildrenWithProps<Skeleton>,

    #[prop_or_default]
    pub style: &'static str,

    #[prop_or_default]
    pub class: &'static str,
}

#[function_component(SkeletonGroup)]
pub fn skeleton_group(props: &SkeletonGroupProps) -> Html {
    html! { <div style={props.style} class={props.class}>{ for props.children.iter() }</div> }
}
