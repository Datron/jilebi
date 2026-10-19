# ApiNodeHealthResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Http** | **string** | &#x60;OK (&lt;status&gt;)&#x60; or &#x60;FAIL: &lt;reason&gt;&#x60;. | 

## Methods

### NewApiNodeHealthResponse

`func NewApiNodeHealthResponse(http string, ) *ApiNodeHealthResponse`

NewApiNodeHealthResponse instantiates a new ApiNodeHealthResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewApiNodeHealthResponseWithDefaults

`func NewApiNodeHealthResponseWithDefaults() *ApiNodeHealthResponse`

NewApiNodeHealthResponseWithDefaults instantiates a new ApiNodeHealthResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetHttp

`func (o *ApiNodeHealthResponse) GetHttp() string`

GetHttp returns the Http field if non-nil, zero value otherwise.

### GetHttpOk

`func (o *ApiNodeHealthResponse) GetHttpOk() (*string, bool)`

GetHttpOk returns a tuple with the Http field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetHttp

`func (o *ApiNodeHealthResponse) SetHttp(v string)`

SetHttp sets Http field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


