use serde::{Deserialize, Serialize};

/// A watch for events for a form. When the designated event happens, a notification will be published to the specified target. The notification's attributes will include a `formId` key that has the ID of the watched form and an `eventType` key that has the string of the type.
///
/// Messages are sent with at-least-once delivery and are only dropped in extraordinary circumstances. Typically all notifications should be reliably delivered within a few seconds; however, in some situations notifications may be delayed.
///
/// A watch expires seven days after it is created unless it is renewed with [`watches.renew`](https://developers.google.com/forms/api/reference/rest/v1/forms.watches/renew#google.apps.forms.v1.FormsService.RenewWatch).
///
/// [View API](https://developers.google.com/forms/api/reference/rest/v1/forms.watches#resource:-watch)
#[derive(Clone, Serialize, Deserialize)]
pub struct Watch {
    /// Output only. The ID of this watch. See notes on [`CreateWatchRequest.watch_id`](https://developers.google.com/forms/api/reference/rest/v1/forms.watches/create#body.request_body.FIELDS.watch_id).
    pub id: String,
    /// Required. Where to send the notification.
    pub target: WatchTarget,
    /// Required. Which event type to watch for.
    pub event_type: EventType,
    /// Output only. Timestamp of when this was created.
    ///
    /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits. Examples: `"2014-10-02T15:01:23Z"` and `"2014-10-02T15:01:23.045123456Z"`.
    pub create_time: String,
    /// Output only. Timestamp for when this will expire. Each [`watches.renew`](https://developers.google.com/forms/api/reference/rest/v1/forms.watches/renew#google.apps.forms.v1.FormsService.RenewWatch) call resets this to seven days in the future.
    ///
    /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits. Examples: `"2014-10-02T15:01:23Z"` and `"2014-10-02T15:01:23.045123456Z"`.
    pub expire_time: String,
    /// Output only. The most recent error type for an attempted delivery. To begin watching the form again a call can be made to [`watches.renew`](https://developers.google.com/forms/api/reference/rest/v1/forms.watches/renew#google.apps.forms.v1.FormsService.RenewWatch) which also clears this error information.
    pub error_type: ErrorType,
    /// Output only. The current state of the watch. Additional details about suspended watches can be found by checking the `errorType`.
    pub state: State,
}

/// The target for notification delivery.
///
/// [View API](https://developers.google.com/forms/api/reference/rest/v1/forms.watches#watchtarget)
#[derive(Clone, Serialize, Deserialize)]
pub struct WatchTarget {
    /// A Pub/Sub topic. To receive notifications, the topic must grant publish privileges to the Forms service account `serviceAccount:forms-notifications@system.gserviceaccount.com`. Only the project that owns a topic may create a watch with it.
    ///
    /// Pub/Sub delivery guarantees should be considered.
    pub topic: CloudPubsubTopic,
}

/// A Pub/Sub topic.
///
/// [View API](https://developers.google.com/forms/api/reference/rest/v1/forms.watches#cloudpubsubtopic)
#[derive(Clone, Serialize, Deserialize)]
pub struct CloudPubsubTopic {
    /// Required. A fully qualified Pub/Sub topic name to publish the events to. This topic must be owned by the calling project and already exist in Pub/Sub.
    pub topic_name: String,
}

/// Possible event types that can be watched.
///
/// [View API](https://developers.google.com/forms/api/reference/rest/v1/forms.watches#eventtype)
#[derive(Clone, Serialize, Deserialize)]
pub enum EventType {
    /// Unspecified event type. This value should not be used.
    EventTypeUnspecified,
    /// The schema event type. A watch with this event type will be notified about changes to form content and settings.
    Schema,
    /// The responses event type. A watch with this event type will be notified when form responses are submitted.
    Responses,
}

/// Possible error types.
///
/// [View API](https://developers.google.com/forms/api/reference/rest/v1/forms.watches#errortype)
#[derive(Clone, Deserialize, Serialize)]
pub enum ErrorType {
    /// Unspecified error type.
    ErrorTypeUnspecified,
    /// The cloud project does not have access to the form being watched. This occurs if the user has revoked the authorization for your project to access their form(s). Watches with this error will not be retried. To attempt to begin watching the form again a call can be made to [`watches.renew`](https://developers.google.com/forms/api/reference/rest/v1/forms.watches/renew#google.apps.forms.v1.FormsService.RenewWatch).
    ProjectNotAuthorized,
    /// The user that granted access no longer has access to the form being watched. Watches with this error will not be retried. To attempt to begin watching the form again a call can be made to [`watches.renew`](https://developers.google.com/forms/api/reference/rest/v1/forms.watches/renew#google.apps.forms.v1.FormsService.RenewWatch).
    NoUserAccess,
    /// Another type of error has occurred. Whether notifications will continue depends on the watch [`state`](https://developers.google.com/forms/api/reference/rest/v1/forms.watches#Watch.FIELDS.state).
    OtherErrors,
}

/// Possible Watch states.
///
/// [View API](https://developers.google.com/forms/api/reference/rest/v1/forms.watches#state)
#[derive(Clone, Deserialize, Serialize)]
pub enum State {
    /// Unspecified state.
    StateUnspecified,
    /// Watch is active.
    Active,
    /// The watch is suspended due to an error that may be resolved. The watch will continue to exist until it expires. To attempt to reactivate the watch a call can be made to [`watches.renew`](https://developers.google.com/forms/api/reference/rest/v1/forms.watches/renew#google.apps.forms.v1.FormsService.RenewWatch).
    Suspended,
}

/// Create a new watch. If a watch ID is provided, it must be unused. For each invoking project, the per form limit is one watch per [`Watch.EventType`](https://developers.google.com/forms/api/reference/rest/v1/forms.watches#EventType). A watch expires seven days after it is created (see [`Watch.expire_time`](https://developers.google.com/forms/api/reference/rest/v1/forms.watches#Watch.FIELDS.expire_time)).
///
/// * `form_id` - Required. ID of the Form to watch.
/// * `request` - The request body.
///
/// [View API](https://developers.google.com/forms/api/reference/rest/v1/forms.watches/create)
pub fn create(form_id: String, request: CreateWatchRequest) -> Result<Watch, ()> {
    Err(())
}

/// Request body for `create`.
///
/// [View API](https://developers.google.com/forms/api/reference/rest/v1/forms.watches/create#request-body)
#[derive(Clone, Serialize, Deserialize)]
pub struct CreateWatchRequest {
    /// Required. The watch object. No ID should be set on this object; use watchId instead.
    pub watch: Watch,
    /// The ID to use for the watch. If specified, the ID must not already be in use. If not specified, an ID is generated. This value should be 4-63 characters, and valid characters are /[a-z][0-9]-/.
    pub watch_id: String,
}

/// Delete a watch.
///
/// * `form_id` - Required. The ID of the Form.
/// * `watch_id` - Required. The ID of the Watch to delete.
///
/// [View API](https://developers.google.com/forms/api/reference/rest/v1/forms.watches/delete)
pub fn delete(form_id: String, watch_id: String) -> Result<(), ()> {
    Err(())
}

/// Return a list of the watches owned by the invoking project. The maximum number of watches is two: For each invoker, the limit is one for each event type per form.
///
/// * `form_id` - Required. The ID of the Form.
///
/// [View API](https://developers.google.com/forms/api/reference/rest/v1/forms.watches/list)
pub fn list(form_id: String) -> Result<ListWatchesResponse, ()> {
    Err(())
}

/// Response body for `list`.
///
/// [View API](https://developers.google.com/forms/api/reference/rest/v1/forms.watches/list#response-body)
#[derive(Clone, Serialize, Deserialize)]
pub struct ListWatchesResponse {
    /// The returned watches.
    pub watches: Vec<Watch>,
}

/// Renew an existing watch for seven days. The [`state`](https://developers.google.com/forms/api/reference/rest/v1/forms.watches#Watch.FIELDS.state) of the watch after renewal is `ACTIVE`, and the `expireTime` is seven days from the renewal. Renewing a watch in an error state (e.g. `SUSPENDED`) succeeds if the error is no longer present, but fail otherwise. After a watch has expired, watches.renew returns `NOT_FOUND`.
///
/// * `form_id` - Required. The ID of the Form.
/// * `watch_id` - Required. The ID of the Watch to renew.
///
/// [View API](https://developers.google.com/forms/api/reference/rest/v1/forms.watches/renew)
pub fn renew(form_id: String, watch_id: String) -> Result<Watch, ()> {
    Err(())
}
