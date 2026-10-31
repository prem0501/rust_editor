use yew::{Component, Context, Html, Properties, component};

use crate::app::components::file_uploader::FileDetails;
#[derive(PartialEq, Properties)]
pub struct PreviewTileProps {
    details: FileDetails,
}

#[component]
pub fn PreviewTile(props: &PreviewTileProps) -> Html {
    let PreviewTileProps { details } = props;
    let file_type: &str = details.file_type.to_string();
    let src = format!(
        "data:{};base64,{}",
        file_type,
        STANDARD.encode(&details.data)
    );
    html! {
      <div class="preview-tile">
          <p class="preview-name">{ &details.name }</p>
          <div class="preview-media">
              if file_type.contains("image") {
                  <img src={src} />
              } else if file_type.contains("video") {
                  <video controls={true}>
                      <source src={src} type={ file_type }/>
                  </video>
              }
          </div>
      </div>
    }
}
