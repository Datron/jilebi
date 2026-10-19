# ApiMeResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**User** | **map[string]interface{}** | The authenticated user. | 
**Session** | **map[string]interface{}** | The active session. | 

## Methods

### NewApiMeResponse

`func NewApiMeResponse(user map[string]interface{}, session map[string]interface{}, ) *ApiMeResponse`

NewApiMeResponse instantiates a new ApiMeResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewApiMeResponseWithDefaults

`func NewApiMeResponseWithDefaults() *ApiMeResponse`

NewApiMeResponseWithDefaults instantiates a new ApiMeResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetUser

`func (o *ApiMeResponse) GetUser() map[string]interface{}`

GetUser returns the User field if non-nil, zero value otherwise.

### GetUserOk

`func (o *ApiMeResponse) GetUserOk() (*map[string]interface{}, bool)`

GetUserOk returns a tuple with the User field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUser

`func (o *ApiMeResponse) SetUser(v map[string]interface{})`

SetUser sets User field to given value.


### GetSession

`func (o *ApiMeResponse) GetSession() map[string]interface{}`

GetSession returns the Session field if non-nil, zero value otherwise.

### GetSessionOk

`func (o *ApiMeResponse) GetSessionOk() (*map[string]interface{}, bool)`

GetSessionOk returns a tuple with the Session field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSession

`func (o *ApiMeResponse) SetSession(v map[string]interface{})`

SetSession sets Session field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


