//! Contains data models from the [Google Forms API](https://developers.google.com/forms/api/reference/rest).

mod forms;
mod responses;
mod watches;

/// Feedback for a respondent about their response to a question.
struct Feedback {
    /// Required. The main text of the feedback.
    text: String,
    /// Additional information provided as part of the feedback, often used to point the respondent to more reading and resources.
    material: Vec<ExtraMaterial>,
}

/// Supplementary material to the feedback.
struct ExtraMaterial {
    /// Required. The contents of the extra material.
    content: ContentType,
}

/// The contents of the extra material.
enum ContentType {
    /// Text feedback.
    Link(TextLink),
    /// Video feedback.
    Video(VideoLink),
}

/// Link for text.
struct TextLink {
    /// Required. The URI.
    uri: String,
    /// Required. Display text for the URI.
    display_text: String,
}

/// Link to a video.
struct VideoLink {
    /// Required. The display text for the link.
    display_text: String,
    /// The URI of a YouTube video.
    youtube_uri: String,
}
