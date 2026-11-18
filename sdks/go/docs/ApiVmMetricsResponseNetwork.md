# ApiVmMetricsResponseNetwork

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**RxBps** | [**ApiVmMetricsSeries**](ApiVmMetricsSeries.md) |  | 
**TxBps** | [**ApiVmMetricsSeries**](ApiVmMetricsSeries.md) |  | 

## Methods

### NewApiVmMetricsResponseNetwork

`func NewApiVmMetricsResponseNetwork(rxBps ApiVmMetricsSeries, txBps ApiVmMetricsSeries, ) *ApiVmMetricsResponseNetwork`

NewApiVmMetricsResponseNetwork instantiates a new ApiVmMetricsResponseNetwork object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewApiVmMetricsResponseNetworkWithDefaults

`func NewApiVmMetricsResponseNetworkWithDefaults() *ApiVmMetricsResponseNetwork`

NewApiVmMetricsResponseNetworkWithDefaults instantiates a new ApiVmMetricsResponseNetwork object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetRxBps

`func (o *ApiVmMetricsResponseNetwork) GetRxBps() ApiVmMetricsSeries`

GetRxBps returns the RxBps field if non-nil, zero value otherwise.

### GetRxBpsOk

`func (o *ApiVmMetricsResponseNetwork) GetRxBpsOk() (*ApiVmMetricsSeries, bool)`

GetRxBpsOk returns a tuple with the RxBps field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRxBps

`func (o *ApiVmMetricsResponseNetwork) SetRxBps(v ApiVmMetricsSeries)`

SetRxBps sets RxBps field to given value.


### GetTxBps

`func (o *ApiVmMetricsResponseNetwork) GetTxBps() ApiVmMetricsSeries`

GetTxBps returns the TxBps field if non-nil, zero value otherwise.

### GetTxBpsOk

`func (o *ApiVmMetricsResponseNetwork) GetTxBpsOk() (*ApiVmMetricsSeries, bool)`

GetTxBpsOk returns a tuple with the TxBps field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTxBps

`func (o *ApiVmMetricsResponseNetwork) SetTxBps(v ApiVmMetricsSeries)`

SetTxBps sets TxBps field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


