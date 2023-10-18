use leptos::*;

#[component]
fn ThemeEngine(children: Children, touch: bool) -> impl IntoView {
    view! {
        <div class=move || match touch {
            true=>"",
            false=>""
        } style="font-size:3rem; --test:pink;">
            {children()}
        </div>
    }
}

#[component]
fn App() -> impl IntoView {
    view! {
        <ThemeEngine touch={false}>
        <p style="color:var(--test)">"Hello, world!"</p>
    </ThemeEngine>
    }
}

fn main() {
    mount_to_body(|| {
        view! {
            <App/>
        }
    })
}
