# ApiRouteUpdateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Port** | Pointer to **int32** |  | [optional] 
**Privacy** | Pointer to **string** |  | [optional] 
**Emails** | Pointer to **[]string** |  | [optional] 

## Methods

### NewApiRouteUpdateRequest

`func NewApiRouteUpdateRequest() *ApiRouteUpdateRequest`

NewApiRouteUpdateRequest instantiates a new ApiRouteUpdateRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewApiRouteUpdateRequestWithDefaults

`func NewApiRouteUpdateRequestWithDefaults() *ApiRouteUpdateRequest`

NewApiRouteUpdateRequestWithDefaults instantiates a new ApiRouteUpdateRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetPort

`func (o *ApiRouteUpdateRequest) GetPort() int32`

GetPort returns the Port field if non-nil, zero value otherwise.

### GetPortOk

`func (o *ApiRouteUpdateRequest) GetPortOk() (*int32, bool)`

GetPortOk returns a tuple with the Port field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPort

`func (o *ApiRouteUpdateRequest) SetPort(v int32)`

SetPort sets Port field to given value.

### HasPort

`func (o *ApiRouteUpdateRequest) HasPort() bool`

HasPort returns a boolean if a field has been set.

### GetPrivacy

`func (o *ApiRouteUpdateRequest) GetPrivacy() string`

GetPrivacy returns the Privacy field if non-nil, zero value otherwise.

### GetPrivacyOk

`func (o *ApiRouteUpdateRequest) GetPrivacyOk() (*string, bool)`

GetPrivacyOk returns a tuple with the Privacy field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPrivacy

`func (o *ApiRouteUpdateRequest) SetPrivacy(v string)`

SetPrivacy sets Privacy field to given value.

### HasPrivacy

`func (o *ApiRouteUpdateRequest) HasPrivacy() bool`

HasPrivacy returns a boolean if a field has been set.

### GetEmails

`func (o *ApiRouteUpdateRequest) GetEmails() []string`

GetEmails returns the Emails field if non-nil, zero value otherwise.

### GetEmailsOk

`func (o *ApiRouteUpdateRequest) GetEmailsOk() (*[]string, bool)`

GetEmailsOk returns a tuple with the Emails field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEmails

`func (o *ApiRouteUpdateRequest) SetEmails(v []string)`

SetEmails sets Emails field to given value.

### HasEmails

`func (o *ApiRouteUpdateRequest) HasEmails() bool`

HasEmails returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


