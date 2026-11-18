# ApiVmMetricsSeries

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Points** | [**[]ApiVmMetricPoint**](ApiVmMetricPoint.md) |  | 
**Current** | **NullableFloat32** | Last sample in the window. | 
**Average** | **NullableFloat32** |  | 
**Max** | **NullableFloat32** |  | 

## Methods

### NewApiVmMetricsSeries

`func NewApiVmMetricsSeries(points []ApiVmMetricPoint, current NullableFloat32, average NullableFloat32, max NullableFloat32, ) *ApiVmMetricsSeries`

NewApiVmMetricsSeries instantiates a new ApiVmMetricsSeries object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewApiVmMetricsSeriesWithDefaults

`func NewApiVmMetricsSeriesWithDefaults() *ApiVmMetricsSeries`

NewApiVmMetricsSeriesWithDefaults instantiates a new ApiVmMetricsSeries object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetPoints

`func (o *ApiVmMetricsSeries) GetPoints() []ApiVmMetricPoint`

GetPoints returns the Points field if non-nil, zero value otherwise.

### GetPointsOk

`func (o *ApiVmMetricsSeries) GetPointsOk() (*[]ApiVmMetricPoint, bool)`

GetPointsOk returns a tuple with the Points field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPoints

`func (o *ApiVmMetricsSeries) SetPoints(v []ApiVmMetricPoint)`

SetPoints sets Points field to given value.


### GetCurrent

`func (o *ApiVmMetricsSeries) GetCurrent() float32`

GetCurrent returns the Current field if non-nil, zero value otherwise.

### GetCurrentOk

`func (o *ApiVmMetricsSeries) GetCurrentOk() (*float32, bool)`

GetCurrentOk returns a tuple with the Current field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCurrent

`func (o *ApiVmMetricsSeries) SetCurrent(v float32)`

SetCurrent sets Current field to given value.


### SetCurrentNil

`func (o *ApiVmMetricsSeries) SetCurrentNil(b bool)`

 SetCurrentNil sets the value for Current to be an explicit nil

### UnsetCurrent
`func (o *ApiVmMetricsSeries) UnsetCurrent()`

UnsetCurrent ensures that no value is present for Current, not even an explicit nil
### GetAverage

`func (o *ApiVmMetricsSeries) GetAverage() float32`

GetAverage returns the Average field if non-nil, zero value otherwise.

### GetAverageOk

`func (o *ApiVmMetricsSeries) GetAverageOk() (*float32, bool)`

GetAverageOk returns a tuple with the Average field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAverage

`func (o *ApiVmMetricsSeries) SetAverage(v float32)`

SetAverage sets Average field to given value.


### SetAverageNil

`func (o *ApiVmMetricsSeries) SetAverageNil(b bool)`

 SetAverageNil sets the value for Average to be an explicit nil

### UnsetAverage
`func (o *ApiVmMetricsSeries) UnsetAverage()`

UnsetAverage ensures that no value is present for Average, not even an explicit nil
### GetMax

`func (o *ApiVmMetricsSeries) GetMax() float32`

GetMax returns the Max field if non-nil, zero value otherwise.

### GetMaxOk

`func (o *ApiVmMetricsSeries) GetMaxOk() (*float32, bool)`

GetMaxOk returns a tuple with the Max field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMax

`func (o *ApiVmMetricsSeries) SetMax(v float32)`

SetMax sets Max field to given value.


### SetMaxNil

`func (o *ApiVmMetricsSeries) SetMaxNil(b bool)`

 SetMaxNil sets the value for Max to be an explicit nil

### UnsetMax
`func (o *ApiVmMetricsSeries) UnsetMax()`

UnsetMax ensures that no value is present for Max, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


