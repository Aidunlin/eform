use serde::{Deserialize, Serialize};

use super::feedback::Feedback;

/// A Google Forms document. A form is created in Drive, and deleting a form or changing its access protections is done via the [Drive API](https://developers.google.com/drive/api/v3/about-sdk).
///
/// [View API](https://developers.google.com/forms/api/reference/rest/v1/forms#resource:-form)
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Form {
    /// Output only. The form ID.
    pub form_id: String,
    /// Required. The title and description of the form.
    pub info: Info,
    /// The form's settings. This must be updated with [`UpdateSettingsRequest`](https://developers.google.com/forms/api/reference/rest/v1/forms/batchUpdate#UpdateSettingsRequest); it is ignored during `forms.create` and  [`UpdateFormInfoRequest`](https://developers.google.com/forms/api/reference/rest/v1/forms/batchUpdate#UpdateFormInfoRequest).
    pub settings: FormSettings,
    /// Required. A list of the form's items, which can include section headers, questions, embedded media, etc.
    pub items: Vec<Item>,
    /// Output only. The revision ID of the form. Used in the [`WriteControl`](https://developers.google.com/forms/api/reference/rest/v1/forms/batchUpdate#WriteControl) in update requests to identify the revision on which the changes are based.
    ///
    /// The format of the revision ID may change over time, so it should be treated opaquely. A returned revision ID is only guaranteed to be valid for 24 hours after it has been returned and cannot be shared across users. If the revision ID is unchanged between calls, then the form has not changed. Conversely, a changed ID (for the same form and user) usually means the form has been updated; however, a changed ID can also be due to internal factors such as ID format changes.
    pub revision_id: String,
    /// Output only. The form URI to share with responders. This opens a page that allows the user to submit responses but not edit the questions.
    pub responder_uri: String,
    /// Output only. The ID of the linked Google Sheet which is accumulating responses from this Form (if such a Sheet exists).
    pub linked_sheet_id: String,
}

/// The general information for a form.
///
/// [View API](https://developers.google.com/forms/api/reference/rest/v1/forms#info)
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Info {
    /// Required. The title of the form which is visible to responders.
    pub title: String,
    /// Output only. The title of the document which is visible in Drive. If `Info.title` is empty, `documentTitle` may appear in its place in the Google Forms UI and be visible to responders. `documentTitle` can be set on create, but cannot be modified by a batchUpdate request. Please use the [Google Drive API](https://developers.google.com/drive/api/v3/reference/files/update) if you need to programmatically update `documentTitle`.
    pub document_title: String,
    /// The description of the form.
    pub description: String,
}

/// A form's settings.
///
/// [View API](https://developers.google.com/forms/api/reference/rest/v1/forms#formsettings)
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct FormSettings {
    /// Settings related to quiz forms and grading.
    pub quiz_settings: QuizSettings,
}

/// Settings related to quiz forms and grading. These must be updated with the UpdateSettingsRequest.
///
/// [View API](https://developers.google.com/forms/api/reference/rest/v1/forms#quizsettings)
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct QuizSettings {
    /// Whether this form is a quiz or not. When true, responses are graded based on question [`Grading`](https://developers.google.com/forms/api/reference/rest/v1/forms#Grading). Upon setting to false, all question [`Grading`](https://developers.google.com/forms/api/reference/rest/v1/forms#Grading) is deleted.
    pub is_quiz: bool,
}

/// A single item of the form. `kind` defines which kind of item it is.
///
/// [View API](https://developers.google.com/forms/api/reference/rest/v1/forms#item)
#[derive(Clone, Serialize, Deserialize)]
pub struct Item {
    /// The item ID.
    ///
    /// On creation, it can be provided but the ID must not be already used in the form. If not provided, a new ID is assigned.
    pub item_id: String,
    /// The title of the item.
    pub title: String,
    /// The description of the item.
    pub description: String,
    /// Required. The kind of item this is.
    pub kind: ItemKind,
}

/// The kind of item this is.
///
/// [View API](https://developers.google.com/forms/api/reference/rest/v1/forms#item)
#[derive(Clone, Serialize, Deserialize)]
pub enum ItemKind {
    /// Poses a question to the user.
    QuestionItem(QuestionItem),
    /// Poses one or more questions to the user with a single major prompt.
    QuestionGroupItem(QuestionGroupItem),
    /// Starts a new page with a title.
    PageBreakItem(PageBreakItem),
    /// Displays a title and description on the page.
    TextItem(TextItem),
    /// Displays an image on the page.
    ImageItem(ImageItem),
    /// Displays a video on the page.
    VideoItem(VideoItem),
}

/// A form item containing a single question.
///
/// [View API](https://developers.google.com/forms/api/reference/rest/v1/forms#questionitem)
#[derive(Clone, Serialize, Deserialize)]
pub struct QuestionItem {
    /// Required. The displayed question.
    pub question: Question,
    /// The image displayed within the question.
    pub image: Image,
}

/// Any question. The specific type of question is known by its `kind`.
///
/// [View API](https://developers.google.com/forms/api/reference/rest/v1/forms#question)
#[derive(Clone, Serialize, Deserialize)]
pub struct Question {
    /// Read only. The question ID.
    ///
    /// On creation, it can be provided but the ID must not be already used in the form. If not provided, a new ID is assigned.
    pub question_id: String,
    /// Whether the question must be answered in order for a respondent to submit their response.
    pub required: bool,
    /// Grading setup for the question.
    pub grading: Grading,
    /// Required. The type of question offered to a respondent.
    pub kind: QuestionKind,
}

/// The type of question offered to a respondent.
///
/// [View API](https://developers.google.com/forms/api/reference/rest/v1/forms#question)
#[derive(Clone, Serialize, Deserialize)]
pub enum QuestionKind {
    /// A respondent can choose from a pre-defined set of options.
    ChoiceQuestion(ChoiceQuestion),
    /// A respondent can enter a free text response.
    TextQuestion(TextQuestion),
    /// A respondent can choose a number from a range.
    ScaleQuestion(ScaleQuestion),
    /// A respondent can enter a date.
    DateQuestion(DateQuestion),
    /// A respondent can enter a time.
    TimeQuestion(TimeQuestion),
    /// A respondent can upload one or more files.
    FileUploadQuestion(FileUploadQuestion),
    /// A row of a [`QuestionGroupItem`](https://developers.google.com/forms/api/reference/rest/v1/forms#QuestionGroupItem).
    RowQuestion(RowQuestion),
}

/// A radio/checkbox/dropdown question.
///
/// [View API](https://developers.google.com/forms/api/reference/rest/v1/forms#choicequestion)
#[derive(Clone, Serialize, Deserialize)]
pub struct ChoiceQuestion {
    /// Required. The type of choice question.
    pub _type: ChoiceType,
    /// Required. List of options that a respondent must choose from.
    pub options: Vec<Option>,
    /// Whether the options should be displayed in random order for different instances of the quiz. This is often used to prevent cheating by respondents who might be looking at another respondent's screen, or to address bias in a survey that might be introduced by always putting the same options first or last.
    pub shuffle: bool,
}

/// The type of choice.
///
/// [View API](https://developers.google.com/forms/api/reference/rest/v1/forms#choicetype)
#[derive(Clone, Serialize, Deserialize)]
pub enum ChoiceType {
    /// Default value. Unused.
    ChoiceTypeUnspecified,
    /// Radio buttons: All choices are shown to the user, who can only pick one of them.
    Radio,
    /// Checkboxes: All choices are shown to the user, who can pick any number of them.
    Checkbox,
    /// Drop-down menu: The choices are only shown to the user on demand, otherwise only the current choice is shown. Only one option can be chosen.
    DropDown,
}

/// An option for a Choice question.
///
/// [View API](https://developers.google.com/forms/api/reference/rest/v1/forms#option)
#[derive(Clone, Serialize, Deserialize)]
pub struct Option {
    /// Required. The choice as presented to the user.
    pub value: String,
    /// Display image as an option.
    pub image: Image,
    /// Whether the option is "other". Currently only applies to `RADIO` and `CHECKBOX` choice types, but is not allowed in a [`QuestionGroupItem`](https://developers.google.com/forms/api/reference/rest/v1/forms#QuestionGroupItem).
    pub is_other: bool,
    /// Which section to go to if this option is selected. Currently only applies to `RADIO` and `SELECT` choice type, but is not allowed in a [`QuestionGroupItem`](https://developers.google.com/forms/api/reference/rest/v1/forms#QuestionGroupItem).
    pub go_to_section: GoToSection,
}

/// Which section to go to if this option is selected. Currently only applies to `RADIO` and `SELECT` choice type, but is not allowed in a [`QuestionGroupItem`](https://developers.google.com/forms/api/reference/rest/v1/forms#QuestionGroupItem)
///
/// [View API](https://developers.google.com/forms/api/reference/rest/v1/forms#option)
#[derive(Clone, Serialize, Deserialize)]
pub enum GoToSection {
    /// Section navigation type.
    GoToAction(GoToAction),
    /// Item ID of section header to go to.
    GoToSectionId(String),
}

/// Constants for section navigation.
///
/// [View API](https://developers.google.com/forms/api/reference/rest/v1/forms#gotoaction)
#[derive(Clone, Serialize, Deserialize)]
pub enum GoToAction {
    /// Default value. Unused.
    GoToActionUnspecified,
    /// Go to the next section.
    NextSection,
    /// Go back to the beginning of the form.
    RestartForm,
    /// Submit form immediately.
    SubmitForm,
}

/// Data representing an image.
///
/// [View API](https://developers.google.com/forms/api/reference/rest/v1/forms#image)
#[derive(Clone, Serialize, Deserialize)]
pub struct Image {
    /// Output only. A URI from which you can download the image; this is valid only for a limited time.
    pub content_uri: String,
    /// A description of the image that is shown on hover and read by screenreaders.
    pub alt_text: String,
    /// Properties of an image.
    pub properties: MediaProperties,
    /// Input only. The source URI is the URI used to insert the image. The source URI can be empty when fetched.
    pub source_uri: String,
}

/// Properties of the media.
///
/// [View API](https://developers.google.com/forms/api/reference/rest/v1/forms#mediaproperties)
#[derive(Clone, Serialize, Deserialize)]
pub struct MediaProperties {
    /// Position of the media.
    pub alignment: Alignment,
    /// The width of the media in pixels. When the media is displayed, it is scaled to the smaller of this value or the width of the displayed form. The original aspect ratio of the media is preserved. If a width is not specified when the media is added to the form, it is set to the width of the media source. Width must be between 0 and 740, inclusive. Setting width to 0 or unspecified is only permitted when updating the media source.
    pub width: i32,
}

/// Alignment on the page.
///
/// [View API](https://developers.google.com/forms/api/reference/rest/v1/forms#alignment)
#[derive(Clone, Serialize, Deserialize)]
pub enum Alignment {
    /// Default value. Unused.
    AlignmentUnspecified,
    /// Left align.
    Left,
    /// Right align.
    Right,
    /// Center.
    Center,
}

/// A text-based question.
///
/// [View API](https://developers.google.com/forms/api/reference/rest/v1/forms#textquestion)
#[derive(Clone, Serialize, Deserialize)]
pub struct TextQuestion {
    /// Whether the question is a paragraph question or not. If not, the question is a short text question.
    pub paragraph: bool,
}

/// A scale question. The user has a range of numeric values to choose from
///
/// [View API](https://developers.google.com/forms/api/reference/rest/v1/forms#scalequestion)
#[derive(Clone, Serialize, Deserialize)]
pub struct ScaleQuestion {
    /// Required. The lowest possible value for the scale.
    pub low: i32,
    /// Required. The highest possible value for the scale.
    pub high: i32,
    /// The label to display describing the lowest point on the scale.
    pub low_label: String,
    /// The label to display describing the highest point on the scale.
    pub high_label: String,
}

/// A date question. Date questions default to just month + day.
///
/// [View API](https://developers.google.com/forms/api/reference/rest/v1/forms#datequestion)
#[derive(Clone, Serialize, Deserialize)]
pub struct DateQuestion {
    /// Whether to include the time as part of the question.
    pub include_time: bool,
    /// Whether to include the year as part of the question.
    pub include_year: bool,
}

/// A time question.
///
/// [View API](https://developers.google.com/forms/api/reference/rest/v1/forms#timequestion)
#[derive(Clone, Serialize, Deserialize)]
pub struct TimeQuestion {
    /// `true` if the question is about an elapsed time. Otherwise it is about a time of day.
    pub duration: bool,
}

/// A file upload question. The API currently does not support creating file upload questions.
///
/// [View API](https://developers.google.com/forms/api/reference/rest/v1/forms#fileuploadquestion)
#[derive(Clone, Serialize, Deserialize)]
pub struct FileUploadQuestion {
    /// Required. The ID of the Drive folder where uploaded files are stored.
    pub folder_id: String,
    /// File types accepted by this question.
    pub types: Vec<FileType>,
    /// Maximum number of files that can be uploaded for this question in a single response.
    pub max_files: usize,
    /// Maximum number of bytes allowed for any single file uploaded to this question.
    pub max_file_size: i64,
}

/// File types that can be uploaded to a file upload question.
///
/// [View API](https://developers.google.com/forms/api/reference/rest/v1/forms#filetype)
#[derive(Clone, Serialize, Deserialize)]
pub enum FileType {
    /// Default value. Unused.
    FileTypeUnspecified,
    /// No restrictions on type.
    Any,
    /// A Google Docs document.
    Document,
    /// A Google Slides presentation.
    Presentation,
    /// A Google Sheets spreadsheet.
    Spreadsheet,
    /// A drawing.
    Drawing,
    /// A PDF.
    PDF,
    /// An image.
    Image,
    /// A video.
    Video,
    /// An audio file.
    Audio,
}

/// Configuration for a question that is part of a question group.
///
/// [View API](https://developers.google.com/forms/api/reference/rest/v1/forms#rowquestion)
#[derive(Clone, Serialize, Deserialize)]
pub struct RowQuestion {
    /// Required. The title for the single row in the [`QuestionGroupItem`](https://developers.google.com/forms/api/reference/rest/v1/forms#QuestionGroupItem).
    pub title: String,
}

/// Grading for a single question
///
/// [View API](https://developers.google.com/forms/api/reference/rest/v1/forms#grading)
#[derive(Clone, Serialize, Deserialize)]
pub struct Grading {
    /// Required. The maximum number of points a respondent can automatically get for a correct answer. This must not be negative.
    pub point_value: i32,
    /// Required. The answer key for the question. Responses are automatically graded based on this field.
    pub correct_answers: CorrectAnswers,
    /// The feedback displayed for correct responses. This feedback can only be set for multiple choice questions that have correct answers provided.
    pub when_right: Feedback,
    /// The feedback displayed for incorrect responses. This feedback can only be set for multiple choice questions that have correct answers provided.
    pub when_wrong: Feedback,
    /// The feedback displayed for all answers. This is commonly used for short answer questions when a quiz owner wants to quickly give respondents some sense of whether they answered the question correctly before they've had a chance to officially grade the response. General feedback cannot be set for automatically graded multiple choice questions.
    pub general_feedback: Feedback,
}

/// The answer key for a question.
///
/// [View API](https://developers.google.com/forms/api/reference/rest/v1/forms#correctanswers)
#[derive(Clone, Serialize, Deserialize)]
pub struct CorrectAnswers {
    /// A list of correct answers. A quiz response can be automatically graded based on these answers. For single-valued questions, a response is marked correct if it matches any value in this list (in other words, multiple correct answers are possible). For multiple-valued (CHECKBOX) questions, a response is marked correct if it contains exactly the values in this list.
    pub answers: Vec<CorrectAnswer>,
}

/// A single correct answer for a question. For multiple-valued (`CHECKBOX`) questions, several `CorrectAnswer`s may be needed to represent a single correct response option.
///
/// [View API](https://developers.google.com/forms/api/reference/rest/v1/forms#correctanswer)
#[derive(Clone, Serialize, Deserialize)]
pub struct CorrectAnswer {
    /// Required. The correct answer value. See the documentation for [`TextAnswer.value`](https://developers.google.com/forms/api/reference/rest/v1/forms.responses#TextAnswer.FIELDS.value) for details on how various value types are formatted.
    pub value: String,
}

/// Defines a question that comprises multiple questions grouped together.
///
/// [View API](https://developers.google.com/forms/api/reference/rest/v1/forms#questiongroupitem)
#[derive(Clone, Serialize, Deserialize)]
pub struct QuestionGroupItem {
    /// Required. A list of questions that belong in this question group. A question must only belong to one group. The `kind` of the group may affect what types of questions are allowed.
    pub questions: Vec<Question>,
    /// The image displayed within the question group above the specific questions.
    pub image: Image,
    /// The question group is a grid with rows of multiple choice questions that share the same options. When `grid` is set, all questions in the group must be of kind `row`.
    pub grid: Grid,
}

/// A grid of choices (radio or check boxes) with each row constituting a separate question. Each row has the same choices, which are shown as the columns.
///
/// [View API](https://developers.google.com/forms/api/reference/rest/v1/forms#grid)
#[derive(Clone, Serialize, Deserialize)]
pub struct Grid {
    /// Required. The choices shared by each question in the grid. In other words, the values of the columns. Only `CHECK_BOX` and `RADIO` choices are allowed.
    pub columns: ChoiceQuestion,
    /// If `true`, the questions are randomly ordered. In other words, the rows appear in a different order for every respondent.
    pub shuffle_questions: bool,
}

/// A page break. The title and description of this item are shown at the top of the new page.
///
/// [View API](https://developers.google.com/forms/api/reference/rest/v1/forms#pagebreakitem)
#[derive(Clone, Serialize, Deserialize)]
pub struct PageBreakItem;

/// A text item.
///
/// [View API](https://developers.google.com/forms/api/reference/rest/v1/forms#textitem)
#[derive(Clone, Serialize, Deserialize)]
pub struct TextItem;

/// An item containing an image.
///
/// [View API](https://developers.google.com/forms/api/reference/rest/v1/forms#imageitem)
#[derive(Clone, Serialize, Deserialize)]
pub struct ImageItem {
    /// Required. The image displayed in the item.
    pub image: Image,
}

/// An item containing a video.
///
/// [View API](https://developers.google.com/forms/api/reference/rest/v1/forms#videoitem)
#[derive(Clone, Serialize, Deserialize)]
pub struct VideoItem {
    /// Required. The video displayed in the item.
    pub video: Video,
    /// The text displayed below the video.
    pub caption: String,
}

/// Data representing a video.
///
/// [View API](https://developers.google.com/forms/api/reference/rest/v1/forms#video)
#[derive(Clone, Serialize, Deserialize)]
pub struct Video {
    /// Required. A YouTube URI.
    pub youtube_uri: String,
    /// Properties of a video.
    pub properties: MediaProperties,
}

/// Change the form with a batch of updates.
///
/// * `form_id` - Required. The form ID.
/// * `request` - Request body.
///
/// [View API](https://developers.google.com/forms/api/reference/rest/v1/forms/batchUpdate)
pub fn batch_update(
    form_id: String,
    request: BatchUpdateFormRequest,
) -> Result<BatchUpdateFormResponse, ()> {
    Err(())
}

/// Request body for `batch_update`.
///
/// [View API](https://developers.google.com/forms/api/reference/rest/v1/forms/batchUpdate#request-body)
#[derive(Clone, Serialize, Deserialize)]
pub struct BatchUpdateFormRequest {
    /// Whether to return an updated version of the model in the response.
    pub include_form_in_response: bool,
    /// Required. The update requests of this batch.
    pub requests: Vec<Request>,
    /// Provides control over how write requests are executed.
    pub write_control: WriteControl,
}

/// Response body for `batch_update`.
///
/// [View API](https://developers.google.com/forms/api/reference/rest/v1/forms/batchUpdate#response-body)
#[derive(Clone, Serialize, Deserialize)]
pub struct BatchUpdateFormResponse {
    /// Based on the bool request field `includeFormInResponse`, a form with all applied mutations/updates is returned or not. This may be later than the revision ID created by these changes.
    pub form: Form,
    /// The reply of the updates. This maps 1:1 with the update requests, although replies to some requests may be empty.
    pub replies: Vec<Response>,
    /// The updated write control after applying the request.
    pub write_control: WriteControl,
}

/// The kinds of update requests that can be made.
///
/// [View API](https://developers.google.com/forms/api/reference/rest/v1/forms/batchUpdate#request)
#[derive(Clone, Serialize, Deserialize)]
pub struct Request {
    /// The kind of request.
    pub kind: UpdateKind,
}

/// The kind of request.
///
/// [View API](https://developers.google.com/forms/api/reference/rest/v1/forms/batchUpdate#request)
#[derive(Clone, Serialize, Deserialize)]
pub enum UpdateKind {
    /// Update Form's Info.
    UpdateFormInfo(UpdateFormInfoRequest),
    /// Updates the Form's settings.
    UpdateSettings(UpdateSettingsRequest),
    /// Create a new item.
    CreateItem(CreateItemRequest),
    /// Move an item to a specified location.
    MoveItem(MoveItemRequest),
    /// Delete an item.
    DeleteItem(DeleteItemRequest),
    /// Update an item.
    UpdateItem(UpdateItemRequest),
}

/// Update Form's Info.
///
/// [View API](https://developers.google.com/forms/api/reference/rest/v1/forms/batchUpdate#updateforminforequest)
#[derive(Clone, Serialize, Deserialize)]
pub struct UpdateFormInfoRequest {
    /// The info to update.
    pub info: Info,
    /// Required. Only values named in this mask are changed. At least one field must be specified. The root `info` is implied and should not be specified. A single `"*"` can be used as short-hand for updating every field.
    ///
    /// This is a comma-separated list of fully qualified names of fields. Example: `"user.displayName,photo"`.
    pub update_mask: String,
}

/// Update Form's [`FormSettings`](https://developers.google.com/forms/api/reference/rest/v1/forms#FormSettings).
///
/// [View API](https://developers.google.com/forms/api/reference/rest/v1/forms/batchUpdate#updatesettingsrequest)
#[derive(Clone, Serialize, Deserialize)]
pub struct UpdateSettingsRequest {
    /// Required. The settings to update with.
    pub settings: FormSettings,
    /// Required. Only values named in this mask are changed. At least one field must be specified. The root `settings` is implied and should not be specified. A single `"*"` can be used as short-hand for updating every field.
    ///
    /// This is a comma-separated list of fully qualified names of fields. Example: `"user.displayName,photo"`.
    pub update_mask: String,
}

/// Create an item in a form.
///
/// [View API](https://developers.google.com/forms/api/reference/rest/v1/forms/batchUpdate#createitemrequest)
#[derive(Clone, Serialize, Deserialize)]
pub struct CreateItemRequest {
    /// Required. The item to create.
    pub item: Item,
    /// Required. Where to place the new item.
    pub location: Location,
}

/// A specific location in a form.
///
/// [View API](https://developers.google.com/forms/api/reference/rest/v1/forms/batchUpdate#location)
#[derive(Clone, Serialize, Deserialize)]
pub struct Location {
    /// The index of an item in the form. This must be in the range
    ///
    /// `[0..N)`
    ///
    /// , where *N* is the number of items in the form.
    pub index: i32,
}

/// Move an item in a form.
///
/// [View API](https://developers.google.com/forms/api/reference/rest/v1/forms/batchUpdate#moveitemrequest)
#[derive(Clone, Serialize, Deserialize)]
pub struct MoveItemRequest {
    /// Required. The location of the item to move.
    pub original_location: Location,
    /// Required. The new location for the item.
    pub new_location: Location,
}

/// Delete an item in a form.
///
/// [View API](https://developers.google.com/forms/api/reference/rest/v1/forms/batchUpdate#deleteitemrequest)
#[derive(Clone, Serialize, Deserialize)]
pub struct DeleteItemRequest {
    /// Required. The location of the item to delete.
    pub location: Location,
}

/// Update an item in a form.
///
/// [View API](https://developers.google.com/forms/api/reference/rest/v1/forms/batchUpdate#updateitemrequest)
#[derive(Clone, Serialize, Deserialize)]
pub struct UpdateItemRequest {
    /// Required. New values for the item. Note that item and question IDs are used if they are provided (and are in the field mask). If an ID is blank (and in the field mask) a new ID is generated. This means you can modify an item by getting the form via [`forms.get`](https://developers.google.com/forms/api/reference/rest/v1/forms/get#google.apps.forms.v1.FormsService.GetForm), modifying your local copy of that item to be how you want it, and using [`UpdateItemRequest`](https://developers.google.com/forms/api/reference/rest/v1/forms/batchUpdate#UpdateItemRequest) to write it back, with the IDs being the same (or not in the field mask).
    pub item: Item,
    /// Required. The location identifying the item to update.
    pub location: Location,
    /// Required. Only values named in this mask are changed.
    ///
    /// This is a comma-separated list of fully qualified names of fields. Example: `"user.displayName,photo"`.
    pub update_mask: String,
}

/// Provides control over how write requests are executed.
///
/// [View API](https://developers.google.com/forms/api/reference/rest/v1/forms/batchUpdate#writecontrol)
#[derive(Clone, Serialize, Deserialize)]
pub struct WriteControl {
    /// Determines the revision of the form from which changes are to be applied, and how the request should behave if that revision is not the current revision of the form.
    pub control: ControlKind,
}

/// Determines the revision of the form from which changes are to be applied, and how the request should behave if that revision is not the current revision of the form.
///
/// [View API](https://developers.google.com/forms/api/reference/rest/v1/forms/batchUpdate#writecontrol)
#[derive(Clone, Serialize, Deserialize)]
pub enum ControlKind {
    /// The revision ID of the form that the write request is applied to. If this is not the latest revision of the form, the request is not processed and returns a 400 bad request error.
    RequiredRevisionId(String),
    /// The target revision ID of the form that the write request is applied to.
    ///
    /// If changes have occurred after this revision, the changes in this update request are transformed against those changes. This results in a new revision of the form that incorporates both the changes in the request and the intervening changes, with the server resolving conflicting changes.
    ///
    /// The target revision ID may only be used to write to recent versions of a form. If the target revision is too far behind the latest revision, the request is not processed and returns a 400 (Bad Request Error). The request may be retried after reading the latest version of the form. In most cases a target revision ID remains valid for several minutes after it is read, but for frequently-edited forms this window may be shorter.
    TargetRevisionId(String),
}

/// A single response from an update.
///
/// [View API](https://developers.google.com/forms/api/reference/rest/v1/forms/batchUpdate#response)
#[derive(Clone, Serialize, Deserialize)]
pub struct Response {
    /// The result of creating an item.
    pub create_item: CreateItemResponse,
}

/// The result of creating an item.
///
/// [View API](https://developers.google.com/forms/api/reference/rest/v1/forms/batchUpdate#createitemresponse)
#[derive(Clone, Serialize, Deserialize)]
pub struct CreateItemResponse {
    /// The ID of the created item.
    pub item_id: String,
    /// The ID of the question created as part of this item, for a question group it lists IDs of all the questions created for this item.
    pub question_id: Vec<String>,
}

/// Create a new form using the title given in the provided form message in the request.
///
/// Important: Only the [`form.info.title`](https://developers.google.com/forms/api/reference/rest/v1/forms#Info.FIELDS.title) and [`form.info.document_title`](https://developers.google.com/forms/api/reference/rest/v1/forms#Info.FIELDS.title) fields are copied to the new form. All other fields including the form description, items and settings are disallowed. To create a new form and add items, you must first call [`forms.create`](https://developers.google.com/forms/api/reference/rest/v1/forms/create#google.apps.forms.v1.FormsService.CreateForm) to create an empty form with a title and (optional) document title, and then call [`forms.update`](https://developers.google.com/forms/api/reference/rest/v1/forms/batchUpdate#google.apps.forms.v1.FormsService.BatchUpdateForm) to add the items.
///
/// * `request` - Request body.
///
/// [View API](https://developers.google.com/forms/api/reference/rest/v1/forms/create)
pub fn create(request: Form) -> Result<Form, ()> {
    Err(())
}

/// Get a form.
///
/// * `form_id` - Required. The form ID.
///
/// [View API](https://developers.google.com/forms/api/reference/rest/v1/forms/get)
pub fn get(form_id: String) -> Result<Form, ()> {
    Err(())
}
