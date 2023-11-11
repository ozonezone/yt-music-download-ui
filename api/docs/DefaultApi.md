# \DefaultApi

All URIs are relative to *http://localhost:3000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_playlist**](DefaultApi.md#get_playlist) | **POST** /playlist | 
[**get_queue**](DefaultApi.md#get_queue) | **POST** /queue | 



## get_playlist

> crate::models::Playlist get_playlist(playlist_get_params)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**playlist_get_params** | [**PlaylistGetParams**](PlaylistGetParams.md) |  | [required] |

### Return type

[**crate::models::Playlist**](Playlist.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_queue

> crate::models::Queue get_queue(queue_get_params)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**queue_get_params** | [**QueueGetParams**](QueueGetParams.md) |  | [required] |

### Return type

[**crate::models::Queue**](Queue.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

