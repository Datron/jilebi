# ApiRouteCreateResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Id** | **string** |  | 
**Name** | **string** |  | 
**MicrovmId** | **string** |  | 
**Privacy** | **string** |  | 
**Port** | **int32** |  | 
**Url** | **string** |  | 
**SharedUsers** | [**[]SharedUser**](SharedUser.md) |  | 

## Methods

### NewApiRouteCreateResponse

`func NewApiRouteCreateResponse(id string, name string, microvmId string, privacy string, port int32, url string, sharedUsers []SharedUser, ) *ApiRouteCreateResponse`

NewApiRouteCreateResponse instantiates a new ApiRouteCreateResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewApiRouteCreateResponseWithDefaults

`func NewApiRouteCreateResponseWithDefaults() *ApiRouteCreateResponse`

NewApiRouteCreateResponseWithDefaults instantiates a new ApiRouteCreateResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetId

`func (o *ApiRouteCreateResponse) GetId() string`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *ApiRouteCreateResponse) GetIdOk() (*string, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *ApiRouteCreateResponse) SetId(v string)`

SetId sets Id field to given value.


### GetName

`func (o *ApiRouteCreateResponse) GetName() string`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *ApiRouteCreateResponse) GetNameOk() (*string, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *ApiRouteCreateResponse) SetName(v string)`

SetName sets Name field to given value.


### GetMicrovmId

`func (o *ApiRouteCreateResponse) GetMicrovmId() string`

GetMicrovmId returns the MicrovmId field if non-nil, zero value otherwise.

### GetMicrovmIdOk

`func (o *ApiRouteCreateResponse) GetMicrovmIdOk() (*string, bool)`

GetMicrovmIdOk returns a tuple with the MicrovmId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMicrovmId

`func (o *ApiRouteCreateResponse) SetMicrovmId(v string)`

SetMicrovmId sets MicrovmId field to given value.


### GetPrivacy

`func (o *ApiRouteCreateResponse) GetPrivacy() string`

GetPrivacy returns the Privacy field if non-nil, zero value otherwise.

### GetPrivacyOk

`func (o *ApiRouteCreateResponse) GetPrivacyOk() (*string, bool)`

GetPrivacyOk returns a tuple with the Privacy field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPrivacy

`func (o *ApiRouteCreateResponse) SetPrivacy(v string)`

SetPrivacy sets Privacy field to given value.


### GetPort

`func (o *ApiRouteCreateResponse) GetPort() int32`

GetPort returns the Port field if non-nil, zero value otherwise.

### GetPortOk

`func (o *ApiRouteCreateResponse) GetPortOk() (*int32, bool)`

GetPortOk returns a tuple with the Port field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPort

`func (o *ApiRouteCreateResponse) SetPort(v int32)`

SetPort sets Port field to given value.


### GetUrl

`func (o *ApiRouteCreateResponse) GetUrl() string`

GetUrl returns the Url field if non-nil, zero value otherwise.

### GetUrlOk

`func (o *ApiRouteCreateResponse) GetUrlOk() (*string, bool)`

GetUrlOk returns a tuple with the Url field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUrl

`func (o *ApiRouteCreateResponse) SetUrl(v string)`

SetUrl sets Url field to given value.


### GetSharedUsers

`func (o *ApiRouteCreateResponse) GetSharedUsers() []SharedUser`

GetSharedUsers returns the SharedUsers field if non-nil, zero value otherwise.

### GetSharedUsersOk

`func (o *ApiRouteCreateResponse) GetSharedUsersOk() (*[]SharedUser, bool)`

GetSharedUsersOk returns a tuple with the SharedUsers field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSharedUsers

`func (o *ApiRouteCreateResponse) SetSharedUsers(v []SharedUser)`

SetSharedUsers sets SharedUsers field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


