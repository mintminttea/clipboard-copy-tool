use dioxus::prelude::*;
use dioxus::desktop::{Config, WindowBuilder, LogicalSize, use_window}; // Added LogicalSize
use arboard::Clipboard;
use base64::engine::{Engine, general_purpose};
use dioxus::desktop::tao::window::Icon;


const TAILWIND_CSS: &str = include_str!("../assets/tailwind.css");
const MAIN_CSS: &str = include_str!("../assets/main.css");


fn main() {
    // Load the icon bytes at compile time and create an Icon
    let icon_bytes = include_bytes!("../icons/icon.ico");
    let image = image::load_from_memory(icon_bytes)
        .expect("Failed to load icon")
        .into_rgba8();
    let (width, height) = image.dimensions();
    let icon = dioxus::desktop::tao::window::Icon::from_rgba(image.into_raw(), width, height).ok();

    // Set a smaller, fixed window size
    let window_size = LogicalSize::new(450.0, 500.0); // Compact size


    let config = Config::default()
        .with_window(
            WindowBuilder::new()
                .with_title("Clipboard File Copy Tool")
                .with_inner_size(window_size) // Compact size
                .with_max_inner_size(window_size)
                .with_min_inner_size(window_size) // Prevent resizing to maintain layout integrity
                .with_resizable(true) // Keeps the UI consistent
                .with_decorations(true) // Removes title bar and borders for a cleaner look 
                .with_window_icon(icon) // Set the custom icon
        ).with_menu(None); // Remove default menu for a cleaner UI

    LaunchBuilder::desktop()
        .with_cfg(config)
        .launch(App);
}

#[component]
fn App() -> Element {
    let window = use_window();
    let window_size = LogicalSize::new(450.0, 500.0);
    use_effect(move || {
        window.set_inner_size(window_size);
        window.set_resizable(false); // 喺呢度鎖死佢
    });
    rsx! {
        document::Style {
            "html, body {{ 
                overflow: hidden; 
                margin: 0; 
                padding: 0; 
                background-color: #f9fafb; 
                height: 100vh;
                width: 100vw;
                user-select: none; // Prevent text selection for a cleaner UX
            }}"
        }
        document::Style { "{TAILWIND_CSS}" }
        document::Style { "{MAIN_CSS}" }
        div { class: "bg-gray-50 min-h-screen p-6", Hero {} }
    }
}

#[component]
pub fn Hero() -> Element {
    let mut file_path = use_signal(|| "No file selected".to_string());
    let mut file_content_set = use_signal(|| false);
    let mut save_button_text = use_signal(|| "Paste & Save File".to_string());

    rsx! {
        div {
            //make the item evenly spaced and centered
            class: "flex flex-col space-y-6 items-stretch max-w-md mx-auto mr-10",
            div { class: "bg-white p-4 rounded-xl shadow-sm border border-gray-200",
                h2 { class: "text-sm font-bold text-gray-400 mb-3 uppercase tracking-wider",
                    "Encode to Base64"
                }
                div {
                    id: "selection-area",
                    class: "flex flex-col items-center justify-center p-6 border-2 border-dashed border-blue-200 rounded-lg cursor-pointer hover:bg-blue-50 transition-colors",
                    onclick: move |_| async move {
                        file_content_set.set(false);
                        if let Some(file_handle) = rfd::AsyncFileDialog::new()
                            .set_title("Select any file")
                            .pick_file()
                            .await
                        {
                            file_path.set(file_handle.path().to_string_lossy().to_string());
                        }
                    },

                    if file_path() == "No file selected" {
                        p { class: "text-gray-500 text-sm", "Click to browse files" }
                    } else {
                        div { class: "text-center overflow-hidden w-full",
                            p { class: "text-blue-600 font-medium text-sm truncate",
                                "{file_path}"
                            }
                        }
                    }
                }

                button {
                    class: "w-full mt-4 py-2 px-4 rounded-lg font-semibold transition-all",
                    style: if file_content_set() { "background-color: #10b981; color: white;" } else { "background-color: #3b82f6; color: white;" },
                    disabled: file_path() == "No file selected",
                    onclick: move |_| {
                        if let Ok(bytes) = std::fs::read(file_path()) {
                            let b64_string = general_purpose::STANDARD.encode(bytes);
                            if let Ok(mut clipboard) = Clipboard::new() {
                                if clipboard.set_text(b64_string).is_ok() {
                                    file_content_set.set(true);
                                }
                            }
                        }
                    },
                    if file_content_set() {
                        "✓ Copied!"
                    } else {
                        "Copy File as Base64"
                    }
                }
            }

            // Separator
            div { class: "relative",
                div { class: "absolute inset-0 flex items-center",
                    div { class: "w-full border-t border-gray-200" }
                }
                div { class: "relative flex justify-center text-xs uppercase",
                    span { class: "bg-gray-50 px-2 text-gray-400", "OR" }
                }
            }

            // Section 2: Base64 to File (Decode)
            div { class: "bg-white p-4 rounded-xl shadow-sm border border-gray-200",
                h2 { class: "text-sm font-bold text-gray-400 mb-3 uppercase tracking-wider",
                    "Decode from Clipboard"
                }

                button {
                    class: "w-full py-3 px-4 bg-green-600 hover:bg-green-700 text-white font-semibold rounded-lg shadow-md transition-all active:transform active:scale-95",
                    onclick: move |_| {
                        async move {
                            if let Ok(mut clipboard) = Clipboard::new() {
                                if let Ok(b64_string) = clipboard.get_text() {
                                    if let Ok(bytes) = general_purpose::STANDARD
                                        .decode(b64_string.trim())
                                    {
                                        if let Some(save_path) = rfd::AsyncFileDialog::new()
                                            .set_title("Save decoded file")
                                            .save_file()
                                            .await
                                        {
                                            if std::fs::write(save_path.path(), bytes).is_ok() {
                                                save_button_text.set("✓ File Saved!".to_string());
                                            }
                                        }
                                    } else {
                                        save_button_text.set("Invalid Base64 in Clipboard".to_string());
                                    }
                                }
                            }
                        }
                    },
                    "{save_button_text}"
                }
                p { class: "text-[10px] text-gray-400 mt-2 text-center",
                    "Reads Base64 from clipboard and saves as file"
                }
            }
        }
    }
}