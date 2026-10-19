# ApiRouteListResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Owned** | [**[]ApiRoute**](ApiRoute.md) |  | 
**Shared** | [**[]ApiRoute**](ApiRoute.md) |  | 

## Methods

### NewApiRouteListResponse

`func NewApiRouteListResponse(owned []ApiRoute, shared []ApiRoute, ) *ApiRouteListResponse`

NewApiRouteListResponse instantiates a new ApiRouteListResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewApiRouteListResponseWithDefaults

`func NewApiRouteListResponseWithDefaults() *ApiRouteListResponse`

NewApiRouteListResponseWithDefaults instantiates a new ApiRouteListResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetOwned

`func (o *ApiRouteListResponse) GetOwned() []ApiRoute`

GetOwned returns the Owned field if non-nil, zero value otherwise.

### GetOwnedOk

`func (o *ApiRouteListResponse) GetOwnedOk() (*[]ApiRoute, bool)`

GetOwnedOk returns a tuple with the Owned field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOwned

`func (o *ApiRouteListResponse) SetOwned(v []ApiRoute)`

SetOwned sets Owned field to given value.


### GetShared

`func (o *ApiRouteListResponse) GetShared() []ApiRoute`

GetShared returns the Shared field if non-nil, zero value otherwise.

### GetSharedOk

`func (o *ApiRouteListResponse) GetSharedOk() (*[]ApiRoute, bool)`

GetSharedOk returns a tuple with the Shared field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetShared

`func (o *ApiRouteListResponse) SetShared(v []ApiRoute)`

SetShared sets Shared field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


