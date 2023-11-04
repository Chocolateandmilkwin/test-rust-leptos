use leptos::*;
use leptos_dom::*;

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
fn Node() -> impl IntoView {
    let (positionX, setPositionX) = create_signal(0);
    let (positionY, setPositionY) = create_signal(0);
    let (down, setDown) = create_signal(false);
    let (downX, setDownX) = create_signal(0);
    let (downY, setDownY) = create_signal(0);
    view! {
        <svg x=50 y=50 width=100 height=100>
            <rect on:pointermove=move |e| {
                if (down.get()) {
                    log!("Test {}",e.client_x());
                    setPositionX.set(e.client_x() - downX.get());
                    setPositionY.set(e.client_y() - downY.get());
                }
            } on:pointerdown=move |e| {
                setDown.set(true);
                setDownX.set(e.client_x());
                setDownY.set(e.client_y());
            }on:pointerup=move |_| {
                setDown.set(false);
            } x={move || positionX.get()} y={move || positionY.get()} width=20 height=20></rect>
        </svg>
    }
}

//log!("Testing");

#[component]
fn NodeEditor(children: Children) -> impl IntoView {
    view! {
        <svg width=1000 height=1000>
            {children()}
        </svg>
    }
}

#[component]
fn App() -> impl IntoView {
    let (touch, set_touch) = create_signal(true);

    view! {
        <ThemeEngine touch={touch}>
            <p style="color:var(--test)">"Hello, world6!"</p>
            <button on:click=move |_| {set_touch.set(!touch.get())}>"Toggle Theme "{move || touch.get()}</button>
            <NodeEditor>
                <Node></Node>
            </NodeEditor>
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
