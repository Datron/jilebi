# ApiNodeUpdateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Specialization** | Pointer to **string** |  | [optional] 
**Status** | Pointer to [**ApiNodeStatus**](ApiNodeStatus.md) |  | [optional] 

## Methods

### NewApiNodeUpdateRequest

`func NewApiNodeUpdateRequest() *ApiNodeUpdateRequest`

NewApiNodeUpdateRequest instantiates a new ApiNodeUpdateRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewApiNodeUpdateRequestWithDefaults

`func NewApiNodeUpdateRequestWithDefaults() *ApiNodeUpdateRequest`

NewApiNodeUpdateRequestWithDefaults instantiates a new ApiNodeUpdateRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetSpecialization

`func (o *ApiNodeUpdateRequest) GetSpecialization() string`

GetSpecialization returns the Specialization field if non-nil, zero value otherwise.

### GetSpecializationOk

`func (o *ApiNodeUpdateRequest) GetSpecializationOk() (*string, bool)`

GetSpecializationOk returns a tuple with the Specialization field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSpecialization

`func (o *ApiNodeUpdateRequest) SetSpecialization(v string)`

SetSpecialization sets Specialization field to given value.

### HasSpecialization

`func (o *ApiNodeUpdateRequest) HasSpecialization() bool`

HasSpecialization returns a boolean if a field has been set.

### GetStatus

`func (o *ApiNodeUpdateRequest) GetStatus() ApiNodeStatus`

GetStatus returns the Status field if non-nil, zero value otherwise.

### GetStatusOk

`func (o *ApiNodeUpdateRequest) GetStatusOk() (*ApiNodeStatus, bool)`

GetStatusOk returns a tuple with the Status field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStatus

`func (o *ApiNodeUpdateRequest) SetStatus(v ApiNodeStatus)`

SetStatus sets Status field to given value.

### HasStatus

`func (o *ApiNodeUpdateRequest) HasStatus() bool`

HasStatus returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


