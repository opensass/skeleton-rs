# Y Skeleton RS Yew Usage

Adding Skeleton RS to your project is simple:

1. Make sure your project is set up with **Yew**. Follow their [Getting Started Guide](https://yew.rs/docs/getting-started/introduction) for setup instructions.

1. Add the Skeleton RS component to your dependencies by including it in your `Cargo.toml` file:

   ```sh
   cargo add skeleton-rs --features=yew
   ```

1. Import the `Skeleton` component into your Yew component and start using it in your app.

## 🛠️ Usage

Incorporating Skeleton RS into your Yew application is easy. Follow these steps:

1. Import the `Skeleton` component into your Yew project:

   ```rust
   use yew::prelude::*;
   use skeleton_rs::yew::Skeleton;
   use skeleton_rs::Variant;
   ```

1. Use the `Skeleton` component within your Yew application:

   ```rust
   use yew::prelude::*;
   use skeleton_rs::yew::Skeleton;
   use skeleton_rs::Variant;


   #[function_component(App)]
   pub fn app() -> Html {
       html! {
           <Skeleton
               variant={Variant::Text}
               width="100%"
               height="1.2em"
               animate_on_visible={true}
           />
       }
   }
   ```

## 🔧 Props

### 🧱 Main Props

| Property     | Type        | Description                                        | Default |
| ------------ | ----------- | -------------------------------------------------- | ------- |
| `variant`    | `Variant`   | Visual variant: `Text`, `Circle`, `Rect`, etc.     | `Text`  |
| `animation`  | `Animation` | Animation style: `Pulse`, `Wave`, `None`.          | `Pulse` |
| `theme`      | `Theme`     | Theme for light/dark variants.                     | `Light` |
| `show`       | `bool`      | Manually control visibility of the skeleton.       | `false` |
| `delay_ms`   | `u32`       | Delay before showing the skeleton in milliseconds. | `0`     |
| `infer_size` | `bool`      | Infers width/height from child content if true.    | `false` |
| `responsive` | `bool`      | Enables scaling for responsive layouts.            | `false` |
| `children`   | `Html`      | Content to wrap in skeleton loading.               | `None`  |

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

### 🧠 DOM Utility

| Property   | Type      | Description                                                    | Default |
| ---------- | --------- | -------------------------------------------------------------- | ------- |
| `node_ref` | `NodeRef` | DOM reference used internally (e.g., for visibility tracking). | Default |

## 💡 Notes

- The `Skeleton` component is primarily designed for loading states and placeholder UI.
- `IntersectionObserver` is used when `animate_on_visible` is enabled to improve performance.
- You can control the component via the `show` prop or allow it to handle visibility with internal state.
- Use `infer_size` with children when you want the skeleton to match their dimensions.
- For better performance, delay the appearance of skeletons using `delay_ms` to avoid flicker.
- You can use `custom_style` and `class` to style it further as needed.
