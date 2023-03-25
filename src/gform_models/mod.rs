//! Contains data models from the [Google Forms API](https://developers.google.com/forms/api/reference/rest).

pub mod forms;
pub mod responses;
pub mod watches;

/// Feedback for a respondent about their response to a question.
#[derive(Clone)]
pub struct Feedback {
    /// Required. The main text of the feedback.
    pub text: String,
    /// Additional information provided as part of the feedback, often used to point the respondent to more reading and resources.
    pub material: Vec<ExtraMaterial>,
}

/// Supplementary material to the feedback.
#[derive(Clone)]
pub struct ExtraMaterial {
    /// Required. The contents of the extra material.
    pub content: ContentType,
}

/// The contents of the extra material.
#[derive(Clone)]
pub enum ContentType {
    /// Text feedback.
    Link(TextLink),
    /// Video feedback.
    Video(VideoLink),
}

/// Link for text.
#[derive(Clone)]
pub struct TextLink {
    /// Required. The URI.
    pub uri: String,
    /// Required. Display text for the URI.
    pub display_text: String,
}

/// Link to a video.
#[derive(Clone)]
pub struct VideoLink {
    /// Required. The display text for the link.
    pub display_text: String,
    /// The URI of a YouTube video.
    pub youtube_uri: String,
}
