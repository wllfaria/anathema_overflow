use anathema::backend::tui::TuiBackend;
use anathema::component::Component;
use anathema::runtime::Runtime;
use anathema::state::{List, State, Value};
use anathema::templates::Document;

#[derive(Default)]
struct Dummy;

#[derive(State)]
struct DummyState {
    content: Value<List<String>>,
}

impl Default for DummyState {
    fn default() -> Self {
        Self {
            content: List::from_iter(
                // arbitrary number, just gotta be > height and have an expand on component
                (0..1_000)
                    .map(|_| String::from("Lorem ipsum dolor sit amet."))
                    .collect::<Vec<_>>(),
            ),
        }
    }
}

impl Component for Dummy {
    type Message = ();
    type State = DummyState;
}

fn main() {
    let doc = Document::new("@index");

    let backend = TuiBackend::builder()
        .enable_raw_mode()
        .hide_cursor()
        .finish()
        .unwrap();

    let mut runtime = Runtime::builder(doc, backend);

    runtime
        .register_default::<Dummy>("index", "templates/index.aml")
        .unwrap();

    runtime.finish().unwrap().run();
}
