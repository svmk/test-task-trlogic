use crate::presentation::web::response::html_response::HtmlResponse;
use crate::presentation::web::error::WebError;

#[derive(new)]
pub struct MainPageController {

}

impl MainPageController {
    pub fn index(&self) -> Result<HtmlResponse, WebError> {
        let response = HtmlResponse::new(include_str!("../../../../assets/web/view/main_page.html"));
        return Ok(response);
    }
}