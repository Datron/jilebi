# ApiVmRestoreResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**SourceUrl** | **string** |  | 
**NewName** | Pointer to **string** | Present when the restore created a new microVM. | [optional] 

## Methods

### NewApiVmRestoreResponse

`func NewApiVmRestoreResponse(sourceUrl string, ) *ApiVmRestoreResponse`

NewApiVmRestoreResponse instantiates a new ApiVmRestoreResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewApiVmRestoreResponseWithDefaults

`func NewApiVmRestoreResponseWithDefaults() *ApiVmRestoreResponse`

NewApiVmRestoreResponseWithDefaults instantiates a new ApiVmRestoreResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetSourceUrl

`func (o *ApiVmRestoreResponse) GetSourceUrl() string`

GetSourceUrl returns the SourceUrl field if non-nil, zero value otherwise.

### GetSourceUrlOk

`func (o *ApiVmRestoreResponse) GetSourceUrlOk() (*string, bool)`

GetSourceUrlOk returns a tuple with the SourceUrl field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSourceUrl

`func (o *ApiVmRestoreResponse) SetSourceUrl(v string)`

SetSourceUrl sets SourceUrl field to given value.


### GetNewName

`func (o *ApiVmRestoreResponse) GetNewName() string`

GetNewName returns the NewName field if non-nil, zero value otherwise.

### GetNewNameOk

`func (o *ApiVmRestoreResponse) GetNewNameOk() (*string, bool)`

GetNewNameOk returns a tuple with the NewName field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetNewName

`func (o *ApiVmRestoreResponse) SetNewName(v string)`

SetNewName sets NewName field to given value.

### HasNewName

`func (o *ApiVmRestoreResponse) HasNewName() bool`

HasNewName returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


