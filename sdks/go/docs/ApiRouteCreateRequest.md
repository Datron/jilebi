# ApiRouteCreateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Name** | **string** |  | 
**MicrovmId** | **string** | Id of the VM the route forwards to. | 
**Port** | **int32** |  | 
**Privacy** | **string** |  | 
**Emails** | Pointer to **[]string** | Required when &#x60;privacy&#x60; is &#x60;shared&#x60;. | [optional] 

## Methods

### NewApiRouteCreateRequest

`func NewApiRouteCreateRequest(name string, microvmId string, port int32, privacy string, ) *ApiRouteCreateRequest`

NewApiRouteCreateRequest instantiates a new ApiRouteCreateRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewApiRouteCreateRequestWithDefaults

`func NewApiRouteCreateRequestWithDefaults() *ApiRouteCreateRequest`

NewApiRouteCreateRequestWithDefaults instantiates a new ApiRouteCreateRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetName

`func (o *ApiRouteCreateRequest) GetName() string`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *ApiRouteCreateRequest) GetNameOk() (*string, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *ApiRouteCreateRequest) SetName(v string)`

SetName sets Name field to given value.


### GetMicrovmId

`func (o *ApiRouteCreateRequest) GetMicrovmId() string`

GetMicrovmId returns the MicrovmId field if non-nil, zero value otherwise.

### GetMicrovmIdOk

`func (o *ApiRouteCreateRequest) GetMicrovmIdOk() (*string, bool)`

GetMicrovmIdOk returns a tuple with the MicrovmId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMicrovmId

`func (o *ApiRouteCreateRequest) SetMicrovmId(v string)`

SetMicrovmId sets MicrovmId field to given value.


### GetPort

`func (o *ApiRouteCreateRequest) GetPort() int32`

GetPort returns the Port field if non-nil, zero value otherwise.

### GetPortOk

`func (o *ApiRouteCreateRequest) GetPortOk() (*int32, bool)`

GetPortOk returns a tuple with the Port field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPort

`func (o *ApiRouteCreateRequest) SetPort(v int32)`

SetPort sets Port field to given value.


### GetPrivacy

`func (o *ApiRouteCreateRequest) GetPrivacy() string`

GetPrivacy returns the Privacy field if non-nil, zero value otherwise.

### GetPrivacyOk

`func (o *ApiRouteCreateRequest) GetPrivacyOk() (*string, bool)`

GetPrivacyOk returns a tuple with the Privacy field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPrivacy

`func (o *ApiRouteCreateRequest) SetPrivacy(v string)`

SetPrivacy sets Privacy field to given value.


### GetEmails

`func (o *ApiRouteCreateRequest) GetEmails() []string`

GetEmails returns the Emails field if non-nil, zero value otherwise.

### GetEmailsOk

`func (o *ApiRouteCreateRequest) GetEmailsOk() (*[]string, bool)`

GetEmailsOk returns a tuple with the Emails field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEmails

`func (o *ApiRouteCreateRequest) SetEmails(v []string)`

SetEmails sets Emails field to given value.

### HasEmails

`func (o *ApiRouteCreateRequest) HasEmails() bool`

HasEmails returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


