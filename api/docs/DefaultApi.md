# \DefaultApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_album**](DefaultApi.md#get_album) | **POST** /album | 
[**get_album_by_playlist_id**](DefaultApi.md#get_album_by_playlist_id) | **POST** /albumByPlaylistId | 
[**get_playlist**](DefaultApi.md#get_playlist) | **POST** /playlist | 
[**get_queue**](DefaultApi.md#get_queue) | **POST** /queue | 
[**get_song**](DefaultApi.md#get_song) | **POST** /song | 



## get_album

> crate::models::AlbumResult get_album(album_get_params)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**album_get_params** | [**AlbumGetParams**](AlbumGetParams.md) |  | [required] |

### Return type

[**crate::models::AlbumResult**](AlbumResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_album_by_playlist_id

> crate::models::AlbumResult get_album_by_playlist_id(album_get_by_playlist_id_params)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**album_get_by_playlist_id_params** | [**AlbumGetByPlaylistIdParams**](AlbumGetByPlaylistIdParams.md) |  | [required] |

### Return type

[**crate::models::AlbumResult**](AlbumResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


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


## get_song

> crate::models::Song get_song(song_get_params)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**song_get_params** | [**SongGetParams**](SongGetParams.md) |  | [required] |

### Return type

[**crate::models::Song**](Song.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

