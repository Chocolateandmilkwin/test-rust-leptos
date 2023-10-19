use leptos::*;

#[component]
fn ThemeEngine(children: Children, touch: ReadSignal<bool>) -> impl IntoView {
    view! {
        <div class=move || match touch.get() {
            true=>"test1",
            false=>"test2"
        } style="font-size:3rem; --test:pink;">
            {children()}
        </div>
    }
}

#[component]
fn App() -> impl IntoView {
    let (touch, set_touch) = create_signal(true);

    view! {
        <ThemeEngine touch={touch}>
        <p style="color:var(--test)">"Hello, world!"</p>
        <button on:click=move |_| {set_touch.set(!touch.get())}>"Toggle Theme "{move || touch.get()}</button>
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
