# ApiNodeCreateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Name** | **string** |  | 
**Url** | **string** |  | 
**Specialization** | Pointer to **string** |  | [optional] 

## Methods

### NewApiNodeCreateRequest

`func NewApiNodeCreateRequest(name string, url string, ) *ApiNodeCreateRequest`

NewApiNodeCreateRequest instantiates a new ApiNodeCreateRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewApiNodeCreateRequestWithDefaults

`func NewApiNodeCreateRequestWithDefaults() *ApiNodeCreateRequest`

NewApiNodeCreateRequestWithDefaults instantiates a new ApiNodeCreateRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetName

`func (o *ApiNodeCreateRequest) GetName() string`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *ApiNodeCreateRequest) GetNameOk() (*string, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *ApiNodeCreateRequest) SetName(v string)`

SetName sets Name field to given value.


### GetUrl

`func (o *ApiNodeCreateRequest) GetUrl() string`

GetUrl returns the Url field if non-nil, zero value otherwise.

### GetUrlOk

`func (o *ApiNodeCreateRequest) GetUrlOk() (*string, bool)`

GetUrlOk returns a tuple with the Url field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUrl

`func (o *ApiNodeCreateRequest) SetUrl(v string)`

SetUrl sets Url field to given value.


### GetSpecialization

`func (o *ApiNodeCreateRequest) GetSpecialization() string`

GetSpecialization returns the Specialization field if non-nil, zero value otherwise.

### GetSpecializationOk

`func (o *ApiNodeCreateRequest) GetSpecializationOk() (*string, bool)`

GetSpecializationOk returns a tuple with the Specialization field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSpecialization

`func (o *ApiNodeCreateRequest) SetSpecialization(v string)`

SetSpecialization sets Specialization field to given value.

### HasSpecialization

`func (o *ApiNodeCreateRequest) HasSpecialization() bool`

HasSpecialization returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


