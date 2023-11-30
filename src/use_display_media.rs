use leptos::*;
use wasm_bindgen::{JsValue, JsCast};
use web_sys::{DisplayMediaStreamConstraints, MediaStream};
use crate::use_window::use_window;


/// Get a Resource containing a media stream from the user's display.
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/use_display_media)
///
/// ## Usage
///
/// ```
/// # use leptos::*;
/// # use leptos_use::use_display_media;
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// #   let stream = use_display_media(None);
/// #
/// #  let video_ref = create_node_ref::<leptos::html::Video>();
/// #    create_effect(move |_| match stream.get() {
/// #        Some(Ok(s)) => {
/// #            video_ref.get().expect("video element ref not created").set_src_object(Some(&s));
/// #            video_ref.get().map(|v| v.play());
/// #        }
/// #        Some(Err(e)) => log::error!("Failed to get media stream: {:?}", e),
/// #        None => log::debug!("No stream yet"),
/// #    });
/// #
/// #    view! { <video _ref=video_ref controls=true autoplay=true muted=true></video> }
/// # }
/// ```
pub fn use_display_media<S>(options: S) -> UseDisplayReturn
where
    S: Into<MaybeSignal<Option<DisplayMediaStreamConstraints>>>,
{
    let opts: MaybeSignal<Option<DisplayMediaStreamConstraints>> = options.into();
    create_local_resource(move || opts.with(|o| o.as_ref().cloned()), create_media)
}

async fn create_media(opts: Option<DisplayMediaStreamConstraints>) -> Result<MediaStream, JsValue> {
    let media = use_window()
        .navigator()
        .ok_or_else(|| JsValue::from_str("Failed to access window.navigator"))
        .and_then(|n| n.media_devices())?;

    let promise = match opts {
        Some(o) => media.get_display_media_with_constraints(&o),
        None => media.get_display_media(),
    }?;
    let res = wasm_bindgen_futures::JsFuture::from(promise).await?;
    Ok::<_, JsValue>(MediaStream::unchecked_from_js(res))
}


/// A leptos resource which optionally accepts a [DisplayMediaParamContraints](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.DisplayMediaStreamConstraints.html) 
/// The resource contains a result containing the media stream or the rejected JsValue
type UseDisplayReturn = Resource<Option<DisplayMediaStreamConstraints>, Result<MediaStream, JsValue>>;
