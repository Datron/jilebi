# ApiVmMetricsResponseMemory

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**UsedMb** | [**ApiVmMetricsSeries**](ApiVmMetricsSeries.md) |  | 
**ConfiguredMb** | [**ApiVmMetricsSeries**](ApiVmMetricsSeries.md) |  | 

## Methods

### NewApiVmMetricsResponseMemory

`func NewApiVmMetricsResponseMemory(usedMb ApiVmMetricsSeries, configuredMb ApiVmMetricsSeries, ) *ApiVmMetricsResponseMemory`

NewApiVmMetricsResponseMemory instantiates a new ApiVmMetricsResponseMemory object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewApiVmMetricsResponseMemoryWithDefaults

`func NewApiVmMetricsResponseMemoryWithDefaults() *ApiVmMetricsResponseMemory`

NewApiVmMetricsResponseMemoryWithDefaults instantiates a new ApiVmMetricsResponseMemory object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetUsedMb

`func (o *ApiVmMetricsResponseMemory) GetUsedMb() ApiVmMetricsSeries`

GetUsedMb returns the UsedMb field if non-nil, zero value otherwise.

### GetUsedMbOk

`func (o *ApiVmMetricsResponseMemory) GetUsedMbOk() (*ApiVmMetricsSeries, bool)`

GetUsedMbOk returns a tuple with the UsedMb field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUsedMb

`func (o *ApiVmMetricsResponseMemory) SetUsedMb(v ApiVmMetricsSeries)`

SetUsedMb sets UsedMb field to given value.


### GetConfiguredMb

`func (o *ApiVmMetricsResponseMemory) GetConfiguredMb() ApiVmMetricsSeries`

GetConfiguredMb returns the ConfiguredMb field if non-nil, zero value otherwise.

### GetConfiguredMbOk

`func (o *ApiVmMetricsResponseMemory) GetConfiguredMbOk() (*ApiVmMetricsSeries, bool)`

GetConfiguredMbOk returns a tuple with the ConfiguredMb field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetConfiguredMb

`func (o *ApiVmMetricsResponseMemory) SetConfiguredMb(v ApiVmMetricsSeries)`

SetConfiguredMb sets ConfiguredMb field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


