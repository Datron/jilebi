# ApiSshKeyCreateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Type** | **string** |  | 
**Name** | Pointer to **string** |  | [optional] 
**PublicKey** | Pointer to **string** | Required when &#x60;type&#x60; is &#x60;upload&#x60;. | [optional] 

## Methods

### NewApiSshKeyCreateRequest

`func NewApiSshKeyCreateRequest(type_ string, ) *ApiSshKeyCreateRequest`

NewApiSshKeyCreateRequest instantiates a new ApiSshKeyCreateRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewApiSshKeyCreateRequestWithDefaults

`func NewApiSshKeyCreateRequestWithDefaults() *ApiSshKeyCreateRequest`

NewApiSshKeyCreateRequestWithDefaults instantiates a new ApiSshKeyCreateRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetType

`func (o *ApiSshKeyCreateRequest) GetType() string`

GetType returns the Type field if non-nil, zero value otherwise.

### GetTypeOk

`func (o *ApiSshKeyCreateRequest) GetTypeOk() (*string, bool)`

GetTypeOk returns a tuple with the Type field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetType

`func (o *ApiSshKeyCreateRequest) SetType(v string)`

SetType sets Type field to given value.


### GetName

`func (o *ApiSshKeyCreateRequest) GetName() string`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *ApiSshKeyCreateRequest) GetNameOk() (*string, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *ApiSshKeyCreateRequest) SetName(v string)`

SetName sets Name field to given value.

### HasName

`func (o *ApiSshKeyCreateRequest) HasName() bool`

HasName returns a boolean if a field has been set.

### GetPublicKey

`func (o *ApiSshKeyCreateRequest) GetPublicKey() string`

GetPublicKey returns the PublicKey field if non-nil, zero value otherwise.

### GetPublicKeyOk

`func (o *ApiSshKeyCreateRequest) GetPublicKeyOk() (*string, bool)`

GetPublicKeyOk returns a tuple with the PublicKey field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPublicKey

`func (o *ApiSshKeyCreateRequest) SetPublicKey(v string)`

SetPublicKey sets PublicKey field to given value.

### HasPublicKey

`func (o *ApiSshKeyCreateRequest) HasPublicKey() bool`

HasPublicKey returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


