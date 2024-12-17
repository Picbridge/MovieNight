use yew::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;
use web_sys::js_sys::{Object, Reflect, Array, Function};

#[wasm_bindgen(inline_js = "
export function createNoUiSlider(element, options) {
    noUiSlider.create(element, options);
}
")]
extern "C" {
    #[wasm_bindgen(js_name = createNoUiSlider)]
    fn create_no_ui_slider(element: &JsValue, options: &JsValue);
}

pub enum Msg {
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub min: f64,
    pub max: f64,
    pub step: f64,
    pub start: (f64, f64),
    pub on_update: Callback<(f64, f64)>,
}

pub struct RangeSlider {
    node_ref: NodeRef,
    _update_closure: Option<Closure<dyn FnMut(Array, JsValue)>>,
}

impl Component for RangeSlider {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        RangeSlider {
            node_ref: NodeRef::default(),
            _update_closure: None,
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let slider_element = self.node_ref.cast::<HtmlElement>().unwrap();
            let options = Object::new();

            // Set up the options for noUiSlider
            Reflect::set(
                &options,
                &JsValue::from_str("start"),
                &JsValue::from_serde(&ctx.props().start).unwrap(),
            )
            .unwrap();

            Reflect::set(
                &options,
                &JsValue::from_str("connect"),
                &JsValue::from_bool(true),
            )
            .unwrap();

            let range = {
                let mut range = std::collections::BTreeMap::new();
                range.insert("min", ctx.props().min);
                range.insert("max", ctx.props().max);
                range
            };

            Reflect::set(
                &options,
                &JsValue::from_str("range"),
                &JsValue::from_serde(&range).unwrap(),
            )
            .unwrap();

            Reflect::set(
                &options,
                &JsValue::from_str("step"),
                &JsValue::from_f64(ctx.props().step),
            )
            .unwrap();

            // Create the noUiSlider instance
            create_no_ui_slider(&JsValue::from(slider_element.clone()), &options);

            // Access the noUiSlider instance from the element
            let no_ui_slider = Reflect::get(
                &JsValue::from(slider_element.clone()),
                &JsValue::from_str("noUiSlider"),
            )
            .unwrap();

            // Prepare the closure to handle updates
            let on_update = ctx.props().on_update.clone();

            let update_closure = Closure::wrap(Box::new(move |values: Array, _handle: JsValue| {
                let min_value = values
                    .get(0)
                    .as_string()
                    .unwrap()
                    .parse::<f64>()
                    .unwrap();
                let max_value = values
                    .get(1)
                    .as_string()
                    .unwrap()
                    .parse::<f64>()
                    .unwrap();
                on_update.emit((min_value, max_value));
            }) as Box<dyn FnMut(Array, JsValue)>);

            // Add the 'update' event listener to the slider
            let on_function = Reflect::get(&no_ui_slider, &JsValue::from_str("on"))
                .unwrap()
                .dyn_into::<Function>()
                .unwrap();

            on_function
                .call2(
                    &no_ui_slider,
                    &JsValue::from_str("update"),
                    update_closure.as_ref(),
                )
                .unwrap();

            // Store the closure to keep it alive
            self._update_closure = Some(update_closure);
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div ref={self.node_ref.clone()}></div>
        }
    }
}
