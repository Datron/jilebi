# ApiVmMetricPoint

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**T** | **float32** | Unix epoch seconds (may include fractional seconds). | 
**V** | **float32** |  | 

## Methods

### NewApiVmMetricPoint

`func NewApiVmMetricPoint(t float32, v float32, ) *ApiVmMetricPoint`

NewApiVmMetricPoint instantiates a new ApiVmMetricPoint object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewApiVmMetricPointWithDefaults

`func NewApiVmMetricPointWithDefaults() *ApiVmMetricPoint`

NewApiVmMetricPointWithDefaults instantiates a new ApiVmMetricPoint object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetT

`func (o *ApiVmMetricPoint) GetT() float32`

GetT returns the T field if non-nil, zero value otherwise.

### GetTOk

`func (o *ApiVmMetricPoint) GetTOk() (*float32, bool)`

GetTOk returns a tuple with the T field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetT

`func (o *ApiVmMetricPoint) SetT(v float32)`

SetT sets T field to given value.


### GetV

`func (o *ApiVmMetricPoint) GetV() float32`

GetV returns the V field if non-nil, zero value otherwise.

### GetVOk

`func (o *ApiVmMetricPoint) GetVOk() (*float32, bool)`

GetVOk returns a tuple with the V field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetV

`func (o *ApiVmMetricPoint) SetV(v float32)`

SetV sets V field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


