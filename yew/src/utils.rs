use web_sys::Element;
use yew::utils::window;

pub fn get_root_element() -> Element {
  let document = window()
    .document()
    .expect("should have a document on window");
  let mount_div = document
    .get_element_by_id("yew-root-wrapper")
    .expect("missing element with 'yew-root' id");

  mount_div
}
