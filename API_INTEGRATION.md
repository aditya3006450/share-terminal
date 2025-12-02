# API Integration and Route Creation

This document outlines the process for integrating new APIs into the Share-Terminal application and defining new frontend routes using Yew. We will use the existing `login`, `signup`, and the newly added `logout` functionality as concrete examples.

The application uses a Frontend (Yew/Rust with `wasm-bindgen`) communicating with a Backend (Tauri/Rust) which then interacts with external APIs (if any) or performs system-level operations.

---

## 1. Understanding the Architecture

- **Frontend (`src/`):** Yew components, services (`src/services/api.rs`), and routing (`src/router/router.rs`).
- **Tauri IPC (`wasm_bindgen::JsValue::invoke`):** The bridge between the frontend and backend. Frontend calls `invoke("command_name", args)` to execute Rust functions in the backend.
- **Backend (`src-tauri/`):** Tauri commands (`#[tauri::command]`) defined in `src-tauri/src/lib.rs` and their implementations, often in dedicated modules like `src-tauri/src/api/auth.rs`. These backend functions can then make HTTP requests to external APIs using `reqwest`.

---

## 2. Adding a New API Endpoint (Example: Logout)

Let's walk through adding the `logout` functionality as an example of integrating a new API.

### Step 2.1: Backend Implementation (`src-tauri/`)

#### 2.1.1 Define the API Logic (`src-tauri/src/api/auth.rs`)

1.  **Define Request/Response Structs:**
    For `logout`, we only need a response struct to indicate success.

    ```rust
    // src-tauri/src/api/auth.rs

    use serde::{Deserialize, Serialize};
    // ... other structs

    #[derive(Serialize, Deserialize)]
    pub struct LogoutResponse {
        pub message: String,
    }
    ```

2.  **Implement the Async Function:**
    Create an `async` function that performs the desired backend logic. This could involve making an HTTP request to an external logout endpoint, clearing session data, etc. For our example, we'll simulate a successful logout.

    ```rust
    // src-tauri/src/api/auth.rs

    // ... other functions (login, signup)

    pub async fn logout() -> Result<LogoutResponse, String> {
        // In a real application, you might call an external API here,
        // e.g., to invalidate a session on a server.
        // For this example, we simply simulate success.
        Ok(LogoutResponse {
            message: "Logged out successfully!".to_string(),
        })
    }
    ```

#### 2.1.2 Register the Tauri Command (`src-tauri/src/lib.rs`)

1.  **Import the Response Struct:**
    Add `LogoutResponse` to the imports from `api::auth`.

    ```rust
    // src-tauri/src/lib.rs

    use api::auth::{LoginResponse, SignupResponse, LogoutResponse}; // <-- Added LogoutResponse
    ```

2.  **Define the Tauri Command:**
    Create a `#[tauri::command]` function that acts as the entry point from the frontend. This function will call the actual logic implemented in `src-tauri/src/api/auth.rs`.

    ```rust
    // src-tauri/src/lib.rs

    // ... existing commands (greet, login, signup)

    #[tauri::command]
    async fn logout() -> Result<LogoutResponse, String> { // <-- New command
        api::auth::logout().await
    }
    ```

3.  **Register in `invoke_handler`:**
    Add the new command name to the `tauri::generate_handler!` macro in the `run` function.

    ```rust
    // src-tauri/src/lib.rs

    pub fn run() {
        tauri::Builder::default()
            .plugin(tauri_plugin_opener::init())
            .invoke_handler(tauri::generate_handler![greet, login, signup, logout]) // <-- Added logout
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
    }
    ```

### Step 2.2: Frontend Integration (`src/`)

#### 2.2.1 Define Frontend API Service (`src/services/api.rs`)

1.  **Define Response Struct:**
    Create a `LogoutResponse` struct in the frontend's `api.rs` that matches the backend's response.

    ```rust
    // src/services/api.rs

    use serde::{Deserialize, Serialize};
    use wasm_bindgen::prelude::*;

    // ... existing structs

    #[derive(Deserialize)] // Only Deserialize needed for responses
    pub struct LogoutResponse {
        pub message: String,
    }
    ```

2.  **Create the Frontend Service Function:**
    Implement an `async` function that calls the Tauri `invoke` command with the appropriate command name and arguments.

    ```rust
    // src/services/api.rs

    // ... existing functions (login, signup)

    pub async fn logout() -> Result<LogoutResponse, String> {
        // No arguments needed for a simple logout in this example, so JsValue::NULL
        let result = invoke("logout", JsValue::NULL).await;

        serde_wasm_bindgen::from_value(result).map_err(|e| format!("Logout failed: {}", e))
    }
    ```

#### 2.2.2 Integrate into a Yew Component (Example: `src/pages/home/access/access.rs`)

1.  **Import Dependencies:**
    Bring in the necessary modules and functions.

    ```rust
    // src/pages/home/access/access.rs

    use yew::prelude::*;
    use yew_router::prelude::*; // For Link and use_navigator
    use gloo::console::log; // For debugging output
    use wasm_bindgen_futures::spawn_local; // To run async tasks

    use crate::router::router::Route; // For navigation
    use crate::utils::auth::{is_authenticated, clear_auth_token}; // Auth utilities
    use crate::services::api; // Our API service
    // ... other imports
    ```

2.  **Manage Authentication State:**
    Use `use_state` to track if the user is logged in, and `use_navigator` for redirection. Use `use_effect_with_deps` to initialize or update the login status.

    ```rust
    // src/pages/home/access/access.rs

    #[function_component(Access)]
    pub fn access() -> Html {
        // ... existing states (sidebar_open, dark_mode)
        let is_logged_in = use_state(|| is_authenticated()); // Track auth status
        let navigator = use_navigator().unwrap(); // For redirection

        // Effect to check authentication status on component load
        use_effect_with_deps(
            {
                let is_logged_in = is_logged_in.clone();
                move |_| {
                    is_logged_in.set(is_authenticated());
                    || {} // Return a cleanup function (empty in this case)
                }
            },
            (), // Run once on mount (empty dependencies)
        );

        // ...
    }
    ```

3.  **Implement the Logout Action:**
    Create a callback that triggers the `api::logout` function, clears the local token, updates the UI state, and redirects the user.

    ```rust
    // src/pages/home/access/access.rs

    // ... other callbacks (toggle_sidebar, close_sidebar, toggle_theme)

    let logout_action = {
        let is_logged_in = is_logged_in.clone();
        let navigator = navigator.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default(); // Prevent default button behavior
            let is_logged_in = is_logged_in.clone();
            let navigator = navigator.clone();
            spawn_local(async move {
                match api::logout().await {
                    Ok(response) => {
                        log!(format!("Logout successful: {}", response.message));
                        clear_auth_token(); // Clear token from local storage
                        is_logged_in.set(false); // Update UI state
                        navigator.push(&Route::Login); // Redirect to login
                    }
                    Err(e) => {
                        log!(format!("Logout failed: {}", e));
                        // Even if API logout fails, clear token locally and redirect
                        clear_auth_token();
                        is_logged_in.set(false);
                        navigator.push(&Route::Login);
                    }
                }
            });
        })
    };
    ```

4.  **Conditional Rendering in HTML:**
    Adjust the `html!` macro to display different content based on the `is_logged_in` state. For example, show a logout button when logged in, or login/signup links when logged out. Also, conditionally display restricted content.

    ```html
    // src/pages/home/access/access.rs

    // ... inside the <header> tag
    <div class="topbar-actions">
        // ... theme toggle button
        if *is_logged_in {
            <button class="logout-btn" onclick={logout_action}>{"Logout"}</button>
        } else {
            <Link<Route> to={Route::Login} classes="login-link">{"Login"}</Link<Route>>
            <Link<Route> to={Route::Signup} classes="signup-link">{"Signup"}</Link<Route>>
        }
    </div>

    // ... inside the <main> tag, for content restriction
    <main class="main-content">
        <h1>{"Access Page"}</h1>

        {if *is_logged_in {
            html! {
                // ... Your cards-container content
            }
        } else {
            html! {
                <p>{"Please Login or Signup to access the content."}</p>
            }
        }}
    </main>
    ```

---

## 3. Creating New Frontend Routes

Adding new pages or views to the application requires defining new routes.

1.  **Create Your New Component:**
    First, create the Yew component for your new page. For example, `src/pages/my_new_page/my_new_page.rs`.

    ```rust
    // src/pages/my_new_page/my_new_page.rs

    use yew::prelude::*;

    #[function_component(MyNewPage)]
    pub fn my_new_page() -> Html {
        html! {
            <div>
                <h1>{"Welcome to My New Page!"}</h1>
                <p>{"This is a brand new page in the application."}</p>
            </div>
        }
    }
    ```

2.  **Create `mod.rs` in the New Page Directory:**
    Ensure your new page module is publicly exposed.

    ```rust
    // src/pages/my_new_page/mod.rs

    pub mod my_new_page;
    ```

3.  **Update the Router (`src/router/router.rs`):**

    a.  **Import the new component:**

        ```rust
        // src/router/router.rs

        // ... existing imports
        use crate::pages::my_new_page::my_new_page::MyNewPage; // <-- New import
        ```

    b.  **Add a new variant to the `Route` enum:**
        This defines the path for your new page.

        ```rust
        // src/router/router.rs

        #[derive(Routable, PartialEq, Clone, Debug)]
        pub enum Route {
            // ... existing routes
            #[at("/my-new-page")] // <-- Define the path
            MyNewPage, // <-- New variant
        }
        ```

    c.  **Add a `match` arm in the `switch` function:**
        This tells the router which component to render for the new route.

        ```rust
        // src/router/router.rs

        pub fn switch(route: Route) -> Html {
            match route {
                // ... existing match arms
                Route::MyNewPage => html! { <MyNewPage /> }, // <-- New match arm
            }
        }
        ```

4.  **Navigate to the New Route:**
    You can now navigate to this new route using `<Link<Route> to={Route::MyNewPage}>` in your Yew components or programmatically using `navigator.push(&Route::MyNewPage)`.

---

By following these steps, you can effectively extend both the backend API and frontend routing capabilities of your Share-Terminal application. Remember to maintain consistency in naming conventions and error handling across your new integrations.
