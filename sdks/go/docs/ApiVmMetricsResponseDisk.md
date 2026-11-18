# ApiVmMetricsResponseDisk

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**UsedGb** | [**ApiVmMetricsSeries**](ApiVmMetricsSeries.md) |  | 
**ConfiguredGb** | [**ApiVmMetricsSeries**](ApiVmMetricsSeries.md) |  | 
**ReadGbps** | [**ApiVmMetricsSeries**](ApiVmMetricsSeries.md) |  | 
**WriteGbps** | [**ApiVmMetricsSeries**](ApiVmMetricsSeries.md) |  | 

## Methods

### NewApiVmMetricsResponseDisk

`func NewApiVmMetricsResponseDisk(usedGb ApiVmMetricsSeries, configuredGb ApiVmMetricsSeries, readGbps ApiVmMetricsSeries, writeGbps ApiVmMetricsSeries, ) *ApiVmMetricsResponseDisk`

NewApiVmMetricsResponseDisk instantiates a new ApiVmMetricsResponseDisk object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewApiVmMetricsResponseDiskWithDefaults

`func NewApiVmMetricsResponseDiskWithDefaults() *ApiVmMetricsResponseDisk`

NewApiVmMetricsResponseDiskWithDefaults instantiates a new ApiVmMetricsResponseDisk object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetUsedGb

`func (o *ApiVmMetricsResponseDisk) GetUsedGb() ApiVmMetricsSeries`

GetUsedGb returns the UsedGb field if non-nil, zero value otherwise.

### GetUsedGbOk

`func (o *ApiVmMetricsResponseDisk) GetUsedGbOk() (*ApiVmMetricsSeries, bool)`

GetUsedGbOk returns a tuple with the UsedGb field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUsedGb

`func (o *ApiVmMetricsResponseDisk) SetUsedGb(v ApiVmMetricsSeries)`

SetUsedGb sets UsedGb field to given value.


### GetConfiguredGb

`func (o *ApiVmMetricsResponseDisk) GetConfiguredGb() ApiVmMetricsSeries`

GetConfiguredGb returns the ConfiguredGb field if non-nil, zero value otherwise.

### GetConfiguredGbOk

`func (o *ApiVmMetricsResponseDisk) GetConfiguredGbOk() (*ApiVmMetricsSeries, bool)`

GetConfiguredGbOk returns a tuple with the ConfiguredGb field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetConfiguredGb

`func (o *ApiVmMetricsResponseDisk) SetConfiguredGb(v ApiVmMetricsSeries)`

SetConfiguredGb sets ConfiguredGb field to given value.


### GetReadGbps

`func (o *ApiVmMetricsResponseDisk) GetReadGbps() ApiVmMetricsSeries`

GetReadGbps returns the ReadGbps field if non-nil, zero value otherwise.

### GetReadGbpsOk

`func (o *ApiVmMetricsResponseDisk) GetReadGbpsOk() (*ApiVmMetricsSeries, bool)`

GetReadGbpsOk returns a tuple with the ReadGbps field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetReadGbps

`func (o *ApiVmMetricsResponseDisk) SetReadGbps(v ApiVmMetricsSeries)`

SetReadGbps sets ReadGbps field to given value.


### GetWriteGbps

`func (o *ApiVmMetricsResponseDisk) GetWriteGbps() ApiVmMetricsSeries`

GetWriteGbps returns the WriteGbps field if non-nil, zero value otherwise.

### GetWriteGbpsOk

`func (o *ApiVmMetricsResponseDisk) GetWriteGbpsOk() (*ApiVmMetricsSeries, bool)`

GetWriteGbpsOk returns a tuple with the WriteGbps field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetWriteGbps

`func (o *ApiVmMetricsResponseDisk) SetWriteGbps(v ApiVmMetricsSeries)`

SetWriteGbps sets WriteGbps field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


