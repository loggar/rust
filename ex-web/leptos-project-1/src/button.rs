#[component]
fn Button(text: Text) -> impl IntoView {
    view! {
        <button>{text}</button>
    }
}
