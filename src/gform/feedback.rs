use serde::{Deserialize, Serialize};

/// Feedback for a respondent about their response to a question.
///
/// [View API](https://developers.google.com/forms/api/reference/rest/v1/Feedback)
#[derive(Clone, Serialize, Deserialize)]
pub struct Feedback {
    /// Required. The main text of the feedback.
    pub text: String,
    /// Additional information provided as part of the feedback, often used to point the respondent to more reading and resources.
    pub material: Vec<ExtraMaterial>,
}

/// Supplementary material to the feedback.
///
/// [View API](https://developers.google.com/forms/api/reference/rest/v1/Feedback#extramaterial)
#[derive(Clone, Serialize, Deserialize)]
pub struct ExtraMaterial {
    /// Required. The contents of the extra material.
    pub content: ContentType,
}

/// The contents of the extra material.
///
/// [View API](https://developers.google.com/forms/api/reference/rest/v1/Feedback#extramaterial)
#[derive(Clone, Serialize, Deserialize)]
pub enum ContentType {
    /// Text feedback.
    Link(TextLink),
    /// Video feedback.
    Video(VideoLink),
}

/// Link for text.
///
/// [View API](https://developers.google.com/forms/api/reference/rest/v1/Feedback#textlink)
#[derive(Clone, Serialize, Deserialize)]
pub struct TextLink {
    /// Required. The URI.
    pub uri: String,
    /// Required. Display text for the URI.
    pub display_text: String,
}

/// Link to a video.
///
/// [View API](https://developers.google.com/forms/api/reference/rest/v1/Feedback#videolink)
#[derive(Clone, Serialize, Deserialize)]
pub struct VideoLink {
    /// Required. The display text for the link.
    pub display_text: String,
    /// The URI of a YouTube video.
    pub youtube_uri: String,
}