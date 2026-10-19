# ApiVmRestoreRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**SourceUrl** | **string** |  | 
**Prefault** | Pointer to **bool** |  | [optional] 
**MemoryRestoreMode** | Pointer to **NullableString** |  | [optional] 
**NewName** | Pointer to **string** | Restore into a new microVM with this name. | [optional] 

## Methods

### NewApiVmRestoreRequest

`func NewApiVmRestoreRequest(sourceUrl string, ) *ApiVmRestoreRequest`

NewApiVmRestoreRequest instantiates a new ApiVmRestoreRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewApiVmRestoreRequestWithDefaults

`func NewApiVmRestoreRequestWithDefaults() *ApiVmRestoreRequest`

NewApiVmRestoreRequestWithDefaults instantiates a new ApiVmRestoreRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetSourceUrl

`func (o *ApiVmRestoreRequest) GetSourceUrl() string`

GetSourceUrl returns the SourceUrl field if non-nil, zero value otherwise.

### GetSourceUrlOk

`func (o *ApiVmRestoreRequest) GetSourceUrlOk() (*string, bool)`

GetSourceUrlOk returns a tuple with the SourceUrl field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSourceUrl

`func (o *ApiVmRestoreRequest) SetSourceUrl(v string)`

SetSourceUrl sets SourceUrl field to given value.


### GetPrefault

`func (o *ApiVmRestoreRequest) GetPrefault() bool`

GetPrefault returns the Prefault field if non-nil, zero value otherwise.

### GetPrefaultOk

`func (o *ApiVmRestoreRequest) GetPrefaultOk() (*bool, bool)`

GetPrefaultOk returns a tuple with the Prefault field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPrefault

`func (o *ApiVmRestoreRequest) SetPrefault(v bool)`

SetPrefault sets Prefault field to given value.

### HasPrefault

`func (o *ApiVmRestoreRequest) HasPrefault() bool`

HasPrefault returns a boolean if a field has been set.

### GetMemoryRestoreMode

`func (o *ApiVmRestoreRequest) GetMemoryRestoreMode() string`

GetMemoryRestoreMode returns the MemoryRestoreMode field if non-nil, zero value otherwise.

### GetMemoryRestoreModeOk

`func (o *ApiVmRestoreRequest) GetMemoryRestoreModeOk() (*string, bool)`

GetMemoryRestoreModeOk returns a tuple with the MemoryRestoreMode field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMemoryRestoreMode

`func (o *ApiVmRestoreRequest) SetMemoryRestoreMode(v string)`

SetMemoryRestoreMode sets MemoryRestoreMode field to given value.

### HasMemoryRestoreMode

`func (o *ApiVmRestoreRequest) HasMemoryRestoreMode() bool`

HasMemoryRestoreMode returns a boolean if a field has been set.

### SetMemoryRestoreModeNil

`func (o *ApiVmRestoreRequest) SetMemoryRestoreModeNil(b bool)`

 SetMemoryRestoreModeNil sets the value for MemoryRestoreMode to be an explicit nil

### UnsetMemoryRestoreMode
`func (o *ApiVmRestoreRequest) UnsetMemoryRestoreMode()`

UnsetMemoryRestoreMode ensures that no value is present for MemoryRestoreMode, not even an explicit nil
### GetNewName

`func (o *ApiVmRestoreRequest) GetNewName() string`

GetNewName returns the NewName field if non-nil, zero value otherwise.

### GetNewNameOk

`func (o *ApiVmRestoreRequest) GetNewNameOk() (*string, bool)`

GetNewNameOk returns a tuple with the NewName field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetNewName

`func (o *ApiVmRestoreRequest) SetNewName(v string)`

SetNewName sets NewName field to given value.

### HasNewName

`func (o *ApiVmRestoreRequest) HasNewName() bool`

HasNewName returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


