use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::feedback::Feedback;

/// A form response.
///
/// [View API](https://developers.google.com/forms/api/reference/rest/v1/forms.responses#resource:-formresponse)
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct FormResponse {
    /// Output only. The form ID.
    pub form_id: String,
    /// Output only. The response ID.
    pub response_id: String,
    /// Output only. Timestamp for the first time the response was submitted.
    ///
    /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits. Examples: `"2014-10-02T15:01:23Z"` and `"2014-10-02T15:01:23.045123456Z"`.
    pub create_time: String,
    /// Output only. Timestamp for the most recent time the response was submitted. Does not track changes to grades.
    ///
    /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits. Examples: `"2014-10-02T15:01:23Z"` and `"2014-10-02T15:01:23.045123456Z"`.
    pub last_submitted_time: String,
    /// Output only. The email address (if collected) for the respondent.
    pub respondent_email: String,
    /// Output only. The actual answers to the questions, keyed by questionId.
    pub answers: HashMap<String, Answer>,
    /// Output only. The total number of points the respondent received for their submission. Only set if the form was a quiz and the response was graded. This includes points automatically awarded via autograding adjusted by any manual corrections entered by the form owner.
    pub total_score: Option<f64>,
}

/// The submitted answer for a question.
///
/// [View API](https://developers.google.com/forms/api/reference/rest/v1/forms.responses#answer)
#[derive(Clone, Serialize, Deserialize)]
pub struct Answer {
    /// Output only. The question's ID. See also [`Question.question_id`](https://developers.google.com/forms/api/reference/rest/v1/forms#Question.FIELDS.question_id).
    pub question_id: String,
    /// Output only. The grade for the answer if the form was a quiz.
    pub grade: Grade,
    /// The user's answer.
    pub value: Value,
}

/// The user's answer.
///
/// [View API](https://developers.google.com/forms/api/reference/rest/v1/forms.responses#answer)
#[derive(Clone, Serialize, Deserialize)]
pub enum Value {
    /// Output only. The specific answers as text.
    TextAnswers(TextAnswers),
    /// Output only. The answers to a file upload question.
    FileUploadAnswers(FileUploadAnswers),
}

/// A question's answers as text.
///
/// [View API](https://developers.google.com/forms/api/reference/rest/v1/forms.responses#textanswers)
#[derive(Clone, Serialize, Deserialize)]
pub struct TextAnswers {
    /// Output only. Answers to a question. For multiple-value [`ChoiceQuestions`](https://developers.google.com/forms/api/reference/rest/v1/forms#ChoiceQuestion), each answer is a separate value.
    pub answers: Vec<TextAnswer>,
}

/// An answer to a question represented as text.
///
/// [View API](https://developers.google.com/forms/api/reference/rest/v1/forms.responses#textanswer)
#[derive(Clone, Serialize, Deserialize)]
pub struct TextAnswer {
    /// Output only. The answer value.
    ///
    /// Formatting used for different kinds of question:
    ///
    /// - [`ChoiceQuestion`](https://developers.google.com/forms/api/reference/rest/v1/forms#ChoiceQuestion)
    ///     - `RADIO` or `DROP_DOWN`: A single string corresponding to the option that was selected.
    ///     - `CHECKBOX`: Multiple strings corresponding to each option that was selected.
    /// - [`TextQuestion`](https://developers.google.com/forms/api/reference/rest/v1/forms#TextQuestion): The text that the user entered.
    /// - [`ScaleQuestion`](https://developers.google.com/forms/api/reference/rest/v1/forms#ScaleQuestion): A string containing the number that was selected.
    /// - [`DateQuestion`](https://developers.google.com/forms/api/reference/rest/v1/forms#DateQuestion)
    ///     - Without time or year: MM-DD e.g. "05-19"
    ///     - With year: YYYY-MM-DD e.g. "1986-05-19"
    ///     - With time: MM-DD HH:MM e.g. "05-19 14:51"
    ///     - With year and time: YYYY-MM-DD HH:MM e.g. "1986-05-19 14:51"
    /// - [`TimeQuestion`](https://developers.google.com/forms/api/reference/rest/v1/forms#TimeQuestion): String with time or duration in HH:MM format e.g. "14:51"
    /// - [`RowQuestion`](https://developers.google.com/forms/api/reference/rest/v1/forms#RowQuestion) within [`QuestionGroupItem`](https://developers.google.com/forms/api/reference/rest/v1/forms#QuestionGroupItem): The answer for each row of a [`QuestionGroupItem`](https://developers.google.com/forms/api/reference/rest/v1/forms#QuestionGroupItem) is represented as a separate [`Answer`](https://developers.google.com/forms/api/reference/rest/v1/forms.responses#Answer). Each will contain one string for `RADIO`-type choices or multiple strings for `CHECKBOX` choices.
    pub value: String,
}

/// All submitted files for a FileUpload question.
///
/// [View API](https://developers.google.com/forms/api/reference/rest/v1/forms.responses#fileuploadanswers)
#[derive(Clone, Serialize, Deserialize)]
pub struct FileUploadAnswers {
    /// Output only. All submitted files for a FileUpload question.
    pub answers: Vec<FileUploadAnswer>,
}

/// Info for a single file submitted to a file upload question.
///
/// [View API](https://developers.google.com/forms/api/reference/rest/v1/forms.responses#fileuploadanswer)
#[derive(Clone, Serialize, Deserialize)]
pub struct FileUploadAnswer {
    /// Output only. The ID of the Google Drive file.
    pub file_id: String,
    /// Output only. The file name, as stored in Google Drive on upload.
    pub file_name: String,
    /// Output only. The MIME type of the file, as stored in Google Drive on upload.
    pub mime_type: String,
}

/// Grade information associated with a respondent's answer to a question.
///
/// [View API](https://developers.google.com/forms/api/reference/rest/v1/forms.responses#grade)
#[derive(Clone, Serialize, Deserialize)]
pub struct Grade {
    /// Output only. The numeric score awarded for the answer.
    pub score: f64,
    /// Output only. Whether the question was answered correctly or not. A zero-point score is not enough to infer incorrectness, since a correctly answered question could be worth zero points.
    pub correct: bool,
    /// Output only. Additional feedback given for an answer.
    pub feedback: Feedback,
}

/// Get one response from the form.
///
/// * `form_id` - Required. The form ID.
/// * `response_id` - Required. The response ID within the form.
///
/// [View API](https://developers.google.com/forms/api/reference/rest/v1/forms.responses/get)
pub fn get(form_id: String, response_id: String) -> Result<FormResponse, ()> {
    Err(())
}

/// List a form's responses.
///
/// * `form_id` - Required. ID of the Form whose responses to list.
/// * `query` - Query parameters.
///
/// [View API](https://developers.google.com/forms/api/reference/rest/v1/forms.responses/list)
pub fn list(form_id: String, query: Query) -> Result<ListFormResponsesResponse, ()> {
    Err(())
}

/// [View API](https://developers.google.com/forms/api/reference/rest/v1/forms.responses/list#query-parameters)
#[derive(Clone, Serialize, Deserialize)]
pub struct Query {
    /// Which form responses to return. Currently, the only supported filters are:
    ///
    /// `timestamp > N`
    ///
    /// which means to get all form responses submitted after (but not at) timestamp *N*.
    ///
    /// `timestamp >= N`
    ///
    /// which means to get all form responses submitted at and after timestamp *N*.
    ///
    /// For both supported filters, timestamp must be formatted in RFC3339 UTC "Zulu" format. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
    pub filter: String,
    /// The maximum number of responses to return. The service may return fewer than this value. If unspecified or zero, at most 5000 responses are returned.
    pub page_size: i32,
    /// A page token returned by a previous list response. If this field is set, the form and the values of the filter must be the same as for the original request.
    pub page_token: String,
}

/// Reponse body for `list`.
///
/// [View API](https://developers.google.com/forms/api/reference/rest/v1/forms.responses/list#response-body)
#[derive(Clone, Serialize, Deserialize)]
pub struct ListFormResponsesResponse {
    /// The returned form responses. Note: The `formId` field is not returned in the `FormResponse` object for list requests.
    pub responses: Vec<FormResponse>,
    /// If set, there are more responses. To get the next page of responses, provide this as `pageToken` in a future request.
    pub next_page_token: String,
}
