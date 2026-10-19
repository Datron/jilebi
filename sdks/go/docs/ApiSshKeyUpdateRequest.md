# ApiSshKeyUpdateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Name** | Pointer to **string** | New label for the key. Use an empty string to clear it. | [optional] 

## Methods

### NewApiSshKeyUpdateRequest

`func NewApiSshKeyUpdateRequest() *ApiSshKeyUpdateRequest`

NewApiSshKeyUpdateRequest instantiates a new ApiSshKeyUpdateRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewApiSshKeyUpdateRequestWithDefaults

`func NewApiSshKeyUpdateRequestWithDefaults() *ApiSshKeyUpdateRequest`

NewApiSshKeyUpdateRequestWithDefaults instantiates a new ApiSshKeyUpdateRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetName

`func (o *ApiSshKeyUpdateRequest) GetName() string`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *ApiSshKeyUpdateRequest) GetNameOk() (*string, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *ApiSshKeyUpdateRequest) SetName(v string)`

SetName sets Name field to given value.

### HasName

`func (o *ApiSshKeyUpdateRequest) HasName() bool`

HasName returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


