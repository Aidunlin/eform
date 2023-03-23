use super::Feedback;

/// A form response.
struct FormResponse {
    /// Output only. The form ID.
    form_id: String,
    /// Output only. The response ID.
    response_id: String,
    /// Output only. Timestamp for the first time the response was submitted.
    ///
    /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits. Examples: `"2014-10-02T15:01:23Z"` and `"2014-10-02T15:01:23.045123456Z"`.
    create_time: String,
    /// Output only. Timestamp for the most recent time the response was submitted. Does not track changes to grades.
    ///
    /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits. Examples: `"2014-10-02T15:01:23Z"` and `"2014-10-02T15:01:23.045123456Z"`.
    last_submitted_time: String,
    /// Output only. The email address (if collected) for the respondent.
    respondent_email: String,
    /// Output only. The actual answers to the questions, keyed by questionId.
    answers: Vec<AnswerKeyValue>,
    /// Output only. The total number of points the respondent received for their submission. Only set if the form was a quiz and the response was graded. This includes points automatically awarded via autograding adjusted by any manual corrections entered by the form owner.
    total_score: Option<f64>,
}

struct AnswerKeyValue {
    key: String,
    value: Answer,
}

/// The submitted answer for a question.
struct Answer {
    /// Output only. The question's ID. See also [`Question.question_id`](https://developers.google.com/forms/api/reference/rest/v1/forms#Question.FIELDS.question_id).
    question_id: String,
    /// Output only. The grade for the answer if the form was a quiz.
    grade: Grade,
    /// The user's answer.
    value: Value,
}

/// The user's answer.
enum Value {
    /// Output only. The specific answers as text.
    TextAnswers(TextAnswers),
    /// Output only. The answers to a file upload question.
    FileUploadAnswers(FileUploadAnswers),
}

/// A question's answers as text.
struct TextAnswers {
    /// Output only. Answers to a question. For multiple-value [`ChoiceQuestions`](https://developers.google.com/forms/api/reference/rest/v1/forms#ChoiceQuestion), each answer is a separate value.
    answers: Vec<TextAnswer>,
}

/// An answer to a question represented as text.
struct TextAnswer {
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
    value: String,
}

/// All submitted files for a FileUpload question.
struct FileUploadAnswers {
    /// Output only. All submitted files for a FileUpload question.
    answers: Vec<FileUploadAnswer>,
}

/// Info for a single file submitted to a file upload question.
struct FileUploadAnswer {
    /// Output only. The ID of the Google Drive file.
    file_id: String,
    /// Output only. The file name, as stored in Google Drive on upload.
    file_name: String,
    /// Output only. The MIME type of the file, as stored in Google Drive on upload.
    mime_type: String,
}

/// Grade information associated with a respondent's answer to a question.
struct Grade {
    /// Output only. The numeric score awarded for the answer.
    score: f64,
    /// Output only. Whether the question was answered correctly or not. A zero-point score is not enough to infer incorrectness, since a correctly answered question could be worth zero points.
    correct: bool,
    /// Output only. Additional feedback given for an answer.
    feedback: Feedback,
}

/// Get one response from the form.
/// 
/// * `form_id` - Required. The form ID.
/// * `response_id` - Required. The response ID within the form.
fn get(form_id: String, response_id: String) -> Result<FormResponse, ()> {
    Err(())
}

/// List a form's responses.
/// 
/// * `form_id` - Required. ID of the Form whose responses to list.
/// * `query` - Query parameters.
fn list(form_id: String, query: Query) -> Result<ListFormResponsesResponse, ()> {
    Err(())
}

struct Query {
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
    filter: String,
    /// The maximum number of responses to return. The service may return fewer than this value. If unspecified or zero, at most 5000 responses are returned.
    page_size: i32,
    /// A page token returned by a previous list response. If this field is set, the form and the values of the filter must be the same as for the original request.
    page_token: String,
}

/// Reponse body for `list`.
struct ListFormResponsesResponse {
    /// The returned form responses. Note: The `formId` field is not returned in the `FormResponse` object for list requests.
    responses: Vec<FormResponse>,
    /// If set, there are more responses. To get the next page of responses, provide this as `pageToken` in a future request.
    next_page_token: String,
}
