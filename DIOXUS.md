# 🧬 Skeleton RS Dioxus Usage

Adding Skeleton RS to your project is simple:

1. Make sure your project is set up with **Dioxus**. Refer to the [Dioxus Getting Started Guide](https://dioxuslabs.com/learn/0.6/getting_started) for setup instructions.

1. Add the **skeleton-rs** library to your dependencies by including it in your `Cargo.toml` file:

   ```sh
   cargo add skeleton-rs --features=dio
   ```

1. Import the `Skeleton` component into your Dioxus application.

## 🛠️ Usage

Incorporating Skeleton RS into your **Dioxus** application is straightforward. Follow these steps:

1. Import the `Skeleton` component into your Dioxus project:

   ```rust
   use dioxus::prelude::*;
   use skeleton_rs::dioxus::Skeleton;
   use skeleton_rs::{Variant, Animation, Direction, Theme};
   ```

1. Use the `Skeleton` component within your Dioxus application:

   ```rust
   use dioxus::prelude::*;
   use skeleton_rs::dioxus::Skeleton;
   use skeleton_rs::Variant;

   #[component]
   fn App() -> Element {
       rsx! {
           Skeleton {
               variant: Variant::Text,
               width: "100%",
               height: "1.2em",
           }
       }
   }
   ```

## 🔧 Props

| Property     | Type        | Description                                                            | Default       |
| ------------ | ----------- | ---------------------------------------------------------------------- | ------------- |
| `variant`    | `Variant`   | Visual variant: `Text`, `Circle`, `Rect`, etc.                         | `Text`        |
| `animation`  | `Animation` | Animation style: `Pulse`, `Wave`, `None`.                              | `Pulse`       |
| `direction`  | `Direction` | Animation direction: `LeftToRight`, `RightToLeft`, `TopToBottom`, etc. | `LeftToRight` |
| `theme`      | `Theme`     | Theme for light/dark variants.                                         | `Light`       |
| `show`       | `bool`      | Manually control visibility of the skeleton.                           | `false`       |
| `delay_ms`   | `u32`       | Delay before showing the skeleton in milliseconds.                     | `0`           |
| `infer_size` | `bool`      | Infers width/height from child content if true.                        | `false`       |
| `responsive` | `bool`      | Enables scaling for responsive layouts.                                | `false`       |
| `children`   | `Element`   | Content to wrap in skeleton loading.                                   | `None`        |

### 🎨 Styling Props

| Property        | Type           | Description                        | Default  |
| --------------- | -------------- | ---------------------------------- | -------- |
| `width`         | `&'static str` | Width of the skeleton block.       | `"100%"` |
| `height`        | `&'static str` | Height of the skeleton block.      | `"1em"`  |
| `font_size`     | `Option<&str>` | Font size used for text variant.   | `None`   |
| `border_radius` | `&'static str` | Border radius for rounded shapes.  | `"4px"`  |
| `line_height`   | `&'static str` | Line height of the skeleton block. | `"1"`    |
| `margin`        | `&'static str` | External margin styling.           | `""`     |
| `custom_style`  | `&'static str` | Inline custom styles.              | `""`     |

### ⚙️ Visibility Behavior

| Property             | Type   | Description                                                  | Default |
| -------------------- | ------ | ------------------------------------------------------------ | ------- |
| `animate_on_hover`   | `bool` | Starts animation on hover.                                   | `false` |
| `animate_on_focus`   | `bool` | Starts animation on focus.                                   | `false` |
| `animate_on_active`  | `bool` | Starts animation on active interaction (e.g., click).        | `false` |
| `animate_on_visible` | `bool` | Uses IntersectionObserver to trigger animation when in view. | `false` |

### 📏 Layout Constraints

| Property     | Type           | Description                 | Default |
| ------------ | -------------- | --------------------------- | ------- |
| `max_width`  | `Option<&str>` | Max width of the skeleton.  | `None`  |
| `min_width`  | `Option<&str>` | Min width of the skeleton.  | `None`  |
| `max_height` | `Option<&str>` | Max height of the skeleton. | `None`  |
| `min_height` | `Option<&str>` | Min height of the skeleton. | `None`  |

## 💡 Notes

- The `Skeleton` component is ideal for loading states and placeholder UIs.
- When `animate_on_visible` is enabled, `IntersectionObserver` is used to optimize performance.
- Use the `show` prop to manually toggle visibility or let the component manage it.
- Enable `infer_size` to make the skeleton size itself based on wrapped children.
- For improved UX, use `delay_ms` to avoid flashing placeholders for fast-loading content.
- Customize styles with `custom_style` and regular class/style props.
