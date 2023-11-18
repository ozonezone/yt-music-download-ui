/* tslint:disable */
/* eslint-disable */
// WARNING: This file was auto-generated with tsoa. Please do not modify it. Re-run tsoa to re-generate this file: https://github.com/lukeautry/tsoa
import { Controller, ValidationService, FieldErrors, ValidateError, TsoaRoute, HttpStatusCodeLiteral, TsoaResponse, fetchMiddlewares } from '@tsoa/runtime';
// WARNING: This file was auto-generated with tsoa. Please do not modify it. Re-run tsoa to re-generate this file: https://github.com/lukeautry/tsoa
import { PlaylistController } from './../../source/routes/playlistController.ts';
// WARNING: This file was auto-generated with tsoa. Please do not modify it. Re-run tsoa to re-generate this file: https://github.com/lukeautry/tsoa
import { AlbumController } from './../../source/routes/albumController.ts';
// WARNING: This file was auto-generated with tsoa. Please do not modify it. Re-run tsoa to re-generate this file: https://github.com/lukeautry/tsoa
import { AlbumByPlaylistIdController } from './../../source/routes/albumController.ts';
// WARNING: This file was auto-generated with tsoa. Please do not modify it. Re-run tsoa to re-generate this file: https://github.com/lukeautry/tsoa
import { QueueController } from './../../source/routes/queueController.ts';
// WARNING: This file was auto-generated with tsoa. Please do not modify it. Re-run tsoa to re-generate this file: https://github.com/lukeautry/tsoa
import { SongController } from './../../source/routes/songController.ts';
import type { RequestHandler, Router } from 'express';

// WARNING: This file was auto-generated with tsoa. Please do not modify it. Re-run tsoa to re-generate this file: https://github.com/lukeautry/tsoa

const models: TsoaRoute.Models = {
    "Thumbnail": {
        "dataType": "refObject",
        "properties": {
            "url": {"dataType":"string","required":true},
            "width": {"dataType":"double","required":true},
            "height": {"dataType":"double","required":true},
        },
        "additionalProperties": false,
    },
    // WARNING: This file was auto-generated with tsoa. Please do not modify it. Re-run tsoa to re-generate this file: https://github.com/lukeautry/tsoa
    "ArtistRun": {
        "dataType": "refObject",
        "properties": {
            "name": {"dataType":"string","required":true},
            "id": {"dataType":"union","subSchemas":[{"dataType":"string"},{"dataType":"enum","enums":[null]}],"required":true},
            "type": {"dataType":"union","subSchemas":[{"dataType":"enum","enums":["artist"]},{"dataType":"enum","enums":["channel"]}],"required":true},
        },
        "additionalProperties": false,
    },
    // WARNING: This file was auto-generated with tsoa. Please do not modify it. Re-run tsoa to re-generate this file: https://github.com/lukeautry/tsoa
    "SongArtist": {
        "dataType": "refAlias",
        "type": {"ref":"ArtistRun","validators":{}},
    },
    // WARNING: This file was auto-generated with tsoa. Please do not modify it. Re-run tsoa to re-generate this file: https://github.com/lukeautry/tsoa
    "Album": {
        "dataType": "refObject",
        "properties": {
            "name": {"dataType":"string","required":true},
            "id": {"dataType":"union","subSchemas":[{"dataType":"string"},{"dataType":"enum","enums":[null]}],"required":true},
        },
        "additionalProperties": false,
    },
    // WARNING: This file was auto-generated with tsoa. Please do not modify it. Re-run tsoa to re-generate this file: https://github.com/lukeautry/tsoa
    "LikeStatus": {
        "dataType": "refAlias",
        "type": {"dataType":"union","subSchemas":[{"dataType":"enum","enums":["LIKE"]},{"dataType":"enum","enums":["INDIFFERENT"]},{"dataType":"enum","enums":["DISLIKE"]}],"validators":{}},
    },
    // WARNING: This file was auto-generated with tsoa. Please do not modify it. Re-run tsoa to re-generate this file: https://github.com/lukeautry/tsoa
    "VideoType": {
        "dataType": "refAlias",
        "type": {"dataType":"union","subSchemas":[{"dataType":"enum","enums":["MUSIC_VIDEO_TYPE_OMV"]},{"dataType":"enum","enums":["MUSIC_VIDEO_TYPE_UGC"]},{"dataType":"enum","enums":["MUSIC_VIDEO_TYPE_ATV"]},{"dataType":"enum","enums":["MUSIC_VIDEO_TYPE_PRIVATELY_OWNED_TRACK"]}],"validators":{}},
    },
    // WARNING: This file was auto-generated with tsoa. Please do not modify it. Re-run tsoa to re-generate this file: https://github.com/lukeautry/tsoa
    "MenuTokens": {
        "dataType": "refObject",
        "properties": {
            "add": {"dataType":"union","subSchemas":[{"dataType":"string"},{"dataType":"enum","enums":[null]}],"required":true},
            "remove": {"dataType":"union","subSchemas":[{"dataType":"string"},{"dataType":"enum","enums":[null]}],"required":true},
            "saved": {"dataType":"boolean","required":true},
        },
        "additionalProperties": false,
    },
    // WARNING: This file was auto-generated with tsoa. Please do not modify it. Re-run tsoa to re-generate this file: https://github.com/lukeautry/tsoa
    "TrendChange": {
        "dataType": "refAlias",
        "type": {"dataType":"union","subSchemas":[{"dataType":"enum","enums":["UP"]},{"dataType":"enum","enums":["DOWN"]},{"dataType":"enum","enums":["NEUTRAL"]}],"validators":{}},
    },
    // WARNING: This file was auto-generated with tsoa. Please do not modify it. Re-run tsoa to re-generate this file: https://github.com/lukeautry/tsoa
    "PlaylistItem": {
        "dataType": "refObject",
        "properties": {
            "videoId": {"dataType":"union","subSchemas":[{"dataType":"string"},{"dataType":"enum","enums":[null]}],"required":true},
            "title": {"dataType":"string","required":true},
            "artists": {"dataType":"array","array":{"dataType":"refAlias","ref":"SongArtist"},"required":true},
            "album": {"dataType":"union","subSchemas":[{"ref":"Album"},{"dataType":"enum","enums":[null]}],"required":true},
            "likeStatus": {"dataType":"union","subSchemas":[{"ref":"LikeStatus"},{"dataType":"enum","enums":[null]}],"required":true},
            "thumbnails": {"dataType":"union","subSchemas":[{"dataType":"array","array":{"dataType":"refObject","ref":"Thumbnail"}},{"dataType":"enum","enums":[null]}],"required":true},
            "isAvailable": {"dataType":"union","subSchemas":[{"dataType":"boolean"},{"dataType":"enum","enums":[null]}],"required":true},
            "isExplicit": {"dataType":"union","subSchemas":[{"dataType":"boolean"},{"dataType":"enum","enums":[null]}],"required":true},
            "videoType": {"dataType":"union","subSchemas":[{"ref":"VideoType"},{"dataType":"enum","enums":[null]}],"required":true},
            "duration": {"dataType":"union","subSchemas":[{"dataType":"string"},{"dataType":"enum","enums":[null]}],"required":true},
            "duration_seconds": {"dataType":"union","subSchemas":[{"dataType":"double"},{"dataType":"enum","enums":[null]}],"required":true},
            "setVideoId": {"dataType":"union","subSchemas":[{"dataType":"string"},{"dataType":"enum","enums":[null]}],"required":true},
            "feedbackTokens": {"dataType":"union","subSchemas":[{"ref":"MenuTokens"},{"dataType":"enum","enums":[null]}],"required":true},
            "feedbackToken": {"dataType":"enum","enums":[null],"required":true},
            "rank": {"dataType":"union","subSchemas":[{"dataType":"string"},{"dataType":"enum","enums":[null]}],"required":true},
            "change": {"dataType":"union","subSchemas":[{"ref":"TrendChange"},{"dataType":"enum","enums":[null]}],"required":true},
        },
        "additionalProperties": false,
    },
    // WARNING: This file was auto-generated with tsoa. Please do not modify it. Re-run tsoa to re-generate this file: https://github.com/lukeautry/tsoa
    "ParsedPlaylist": {
        "dataType": "refObject",
        "properties": {
            "shuffleId": {"dataType":"union","subSchemas":[{"dataType":"string"},{"dataType":"enum","enums":[null]}],"required":true},
            "radioId": {"dataType":"union","subSchemas":[{"dataType":"string"},{"dataType":"enum","enums":[null]}],"required":true},
            "type": {"dataType":"enum","enums":["playlist"],"required":true},
            "title": {"dataType":"string","required":true},
            "playlistId": {"dataType":"string","required":true},
            "thumbnails": {"dataType":"array","array":{"dataType":"refObject","ref":"Thumbnail"},"required":true},
            "songs": {"dataType":"union","subSchemas":[{"dataType":"string"},{"dataType":"enum","enums":[null]}],"required":true},
            "authors": {"dataType":"union","subSchemas":[{"dataType":"array","array":{"dataType":"refObject","ref":"ArtistRun"}},{"dataType":"enum","enums":[null]}],"required":true},
            "description": {"dataType":"union","subSchemas":[{"dataType":"string"},{"dataType":"enum","enums":[null]}],"required":true},
            "count": {"dataType":"union","subSchemas":[{"dataType":"string"},{"dataType":"enum","enums":[null]}],"required":true},
            "author": {"dataType":"union","subSchemas":[{"dataType":"array","array":{"dataType":"refObject","ref":"ArtistRun"}},{"dataType":"enum","enums":[null]}],"required":true},
            "libraryLikeStatus": {"dataType":"union","subSchemas":[{"ref":"LikeStatus"},{"dataType":"enum","enums":[null]}],"required":true},
        },
        "additionalProperties": false,
    },
    // WARNING: This file was auto-generated with tsoa. Please do not modify it. Re-run tsoa to re-generate this file: https://github.com/lukeautry/tsoa
    "Playlist": {
        "dataType": "refObject",
        "properties": {
            "id": {"dataType":"string","required":true},
            "privacy": {"dataType":"union","subSchemas":[{"dataType":"enum","enums":["PUBLIC"]},{"dataType":"enum","enums":["PRIVATE"]},{"dataType":"enum","enums":["UNLISTED"]}],"required":true},
            "editable": {"dataType":"boolean","required":true},
            "title": {"dataType":"string","required":true},
            "thumbnails": {"dataType":"array","array":{"dataType":"refObject","ref":"Thumbnail"},"required":true},
            "description": {"dataType":"union","subSchemas":[{"dataType":"string"},{"dataType":"enum","enums":[null]}],"required":true},
            "authors": {"dataType":"array","array":{"dataType":"refObject","ref":"ArtistRun"},"required":true},
            "type": {"dataType":"string","required":true},
            "year": {"dataType":"union","subSchemas":[{"dataType":"string"},{"dataType":"enum","enums":[null]}],"required":true},
            "trackCount": {"dataType":"any","required":true},
            "duration": {"dataType":"union","subSchemas":[{"dataType":"string"},{"dataType":"enum","enums":[null]}],"required":true},
            "duration_seconds": {"dataType":"double","required":true},
            "tracks": {"dataType":"array","array":{"dataType":"refObject","ref":"PlaylistItem"},"required":true},
            "continuation": {"dataType":"union","subSchemas":[{"dataType":"string"},{"dataType":"enum","enums":[null]}],"required":true},
            "suggestions": {"dataType":"array","array":{"dataType":"refObject","ref":"PlaylistItem"},"required":true},
            "suggestions_continuation": {"dataType":"union","subSchemas":[{"dataType":"string"},{"dataType":"enum","enums":[null]}],"required":true},
            "related": {"dataType":"array","array":{"dataType":"refObject","ref":"ParsedPlaylist"},"required":true},
        },
        "additionalProperties": false,
    },
    // WARNING: This file was auto-generated with tsoa. Please do not modify it. Re-run tsoa to re-generate this file: https://github.com/lukeautry/tsoa
    "PlaylistGetParams": {
        "dataType": "refAlias",
        "type": {"dataType":"nestedObjectLiteral","nestedProperties":{"playlistId":{"dataType":"string","required":true}},"validators":{}},
    },
    // WARNING: This file was auto-generated with tsoa. Please do not modify it. Re-run tsoa to re-generate this file: https://github.com/lukeautry/tsoa
    "AlbumType": {
        "dataType": "refAlias",
        "type": {"dataType":"string","validators":{}},
    },
    // WARNING: This file was auto-generated with tsoa. Please do not modify it. Re-run tsoa to re-generate this file: https://github.com/lukeautry/tsoa
    "ParsedAlbum": {
        "dataType": "refObject",
        "properties": {
            "shuffleId": {"dataType":"union","subSchemas":[{"dataType":"string"},{"dataType":"enum","enums":[null]}],"required":true},
            "radioId": {"dataType":"union","subSchemas":[{"dataType":"string"},{"dataType":"enum","enums":[null]}],"required":true},
            "type": {"dataType":"enum","enums":["album"],"required":true},
            "title": {"dataType":"string","required":true},
            "year": {"dataType":"union","subSchemas":[{"dataType":"string"},{"dataType":"enum","enums":[null]}],"required":true},
            "browseId": {"dataType":"string","required":true},
            "audioPlaylistId": {"dataType":"string","required":true},
            "thumbnails": {"dataType":"array","array":{"dataType":"refObject","ref":"Thumbnail"},"required":true},
            "isExplicit": {"dataType":"boolean","required":true},
            "album_type": {"dataType":"union","subSchemas":[{"ref":"AlbumType"},{"dataType":"enum","enums":[null]}],"required":true},
            "artists": {"dataType":"array","array":{"dataType":"refObject","ref":"ArtistRun"},"required":true},
            "libraryLikeStatus": {"dataType":"union","subSchemas":[{"ref":"LikeStatus"},{"dataType":"enum","enums":[null]}],"required":true},
        },
        "additionalProperties": false,
    },
    // WARNING: This file was auto-generated with tsoa. Please do not modify it. Re-run tsoa to re-generate this file: https://github.com/lukeautry/tsoa
    "SongRuns": {
        "dataType": "refObject",
        "properties": {
            "artists": {"dataType":"array","array":{"dataType":"refAlias","ref":"SongArtist"},"required":true},
            "album": {"dataType":"union","subSchemas":[{"ref":"Album"},{"dataType":"enum","enums":[null]}],"required":true},
            "views": {"dataType":"union","subSchemas":[{"dataType":"string"},{"dataType":"enum","enums":[null]}],"required":true},
            "duration": {"dataType":"union","subSchemas":[{"dataType":"string"},{"dataType":"enum","enums":[null]}],"required":true},
            "duration_seconds": {"dataType":"union","subSchemas":[{"dataType":"double"},{"dataType":"enum","enums":[null]}],"required":true},
            "year": {"dataType":"union","subSchemas":[{"dataType":"string"},{"dataType":"enum","enums":[null]}],"required":true},
        },
        "additionalProperties": false,
    },
    // WARNING: This file was auto-generated with tsoa. Please do not modify it. Re-run tsoa to re-generate this file: https://github.com/lukeautry/tsoa
    "MenuPlaylists": {
        "dataType": "refObject",
        "properties": {
            "shuffleId": {"dataType":"union","subSchemas":[{"dataType":"string"},{"dataType":"enum","enums":[null]}],"required":true},
            "radioId": {"dataType":"union","subSchemas":[{"dataType":"string"},{"dataType":"enum","enums":[null]}],"required":true},
        },
        "additionalProperties": false,
    },
    // WARNING: This file was auto-generated with tsoa. Please do not modify it. Re-run tsoa to re-generate this file: https://github.com/lukeautry/tsoa
    "AlbumResult": {
        "dataType": "refObject",
        "properties": {
            "title": {"dataType":"string","required":true},
            "album_type": {"ref":"AlbumType","required":true},
            "thumbnails": {"dataType":"array","array":{"dataType":"refObject","ref":"Thumbnail"},"required":true},
            "isExplicit": {"dataType":"boolean","required":true},
            "description": {"dataType":"union","subSchemas":[{"dataType":"string"},{"dataType":"enum","enums":[null]}],"required":true},
            "trackCount": {"dataType":"union","subSchemas":[{"dataType":"string"},{"dataType":"enum","enums":[null]}],"required":true},
            "duration": {"dataType":"union","subSchemas":[{"dataType":"string"},{"dataType":"enum","enums":[null]}],"required":true},
            "audioPlaylistId": {"dataType":"union","subSchemas":[{"dataType":"string"},{"dataType":"enum","enums":[null]}],"required":true},
            "likeStatus": {"dataType":"union","subSchemas":[{"ref":"LikeStatus"},{"dataType":"enum","enums":[null]}],"required":true},
            "id": {"dataType":"string","required":true},
            "tracks": {"dataType":"array","array":{"dataType":"refObject","ref":"PlaylistItem"},"required":true},
            "other_versions": {"dataType":"union","subSchemas":[{"dataType":"array","array":{"dataType":"refObject","ref":"ParsedAlbum"}},{"dataType":"enum","enums":[null]}],"required":true},
            "artists": {"dataType":"array","array":{"dataType":"refObject","ref":"ArtistRun"}},
        },
        "additionalProperties": false,
    },
    // WARNING: This file was auto-generated with tsoa. Please do not modify it. Re-run tsoa to re-generate this file: https://github.com/lukeautry/tsoa
    "AlbumGetParams": {
        "dataType": "refAlias",
        "type": {"dataType":"nestedObjectLiteral","nestedProperties":{"browseId":{"dataType":"string","required":true}},"validators":{}},
    },
    // WARNING: This file was auto-generated with tsoa. Please do not modify it. Re-run tsoa to re-generate this file: https://github.com/lukeautry/tsoa
    "AlbumGetByPlaylistIdParams": {
        "dataType": "refAlias",
        "type": {"dataType":"nestedObjectLiteral","nestedProperties":{"playlistId":{"dataType":"string","required":true}},"validators":{}},
    },
    // WARNING: This file was auto-generated with tsoa. Please do not modify it. Re-run tsoa to re-generate this file: https://github.com/lukeautry/tsoa
    "QueueChip": {
        "dataType": "refObject",
        "properties": {
            "title": {"dataType":"string","required":true},
            "playlistId": {"dataType":"string","required":true},
            "params": {"dataType":"string","required":true},
        },
        "additionalProperties": false,
    },
    // WARNING: This file was auto-generated with tsoa. Please do not modify it. Re-run tsoa to re-generate this file: https://github.com/lukeautry/tsoa
    "QueueTrack": {
        "dataType": "refObject",
        "properties": {
            "artists": {"dataType":"array","array":{"dataType":"refAlias","ref":"SongArtist"},"required":true},
            "album": {"dataType":"union","subSchemas":[{"ref":"Album"},{"dataType":"enum","enums":[null]}],"required":true},
            "views": {"dataType":"union","subSchemas":[{"dataType":"string"},{"dataType":"enum","enums":[null]}],"required":true},
            "duration": {"dataType":"union","subSchemas":[{"dataType":"string"},{"dataType":"enum","enums":[null]}],"required":true},
            "duration_seconds": {"dataType":"union","subSchemas":[{"dataType":"double"},{"dataType":"enum","enums":[null]}],"required":true},
            "year": {"dataType":"union","subSchemas":[{"dataType":"string"},{"dataType":"enum","enums":[null]}],"required":true},
            "videoId": {"dataType":"string","required":true},
            "title": {"dataType":"string","required":true},
            "thumbnails": {"dataType":"array","array":{"dataType":"refObject","ref":"Thumbnail"},"required":true},
            "feedbackTokens": {"dataType":"union","subSchemas":[{"ref":"MenuTokens"},{"dataType":"enum","enums":[null]}],"required":true},
            "likeStatus": {"dataType":"union","subSchemas":[{"ref":"LikeStatus"},{"dataType":"enum","enums":[null]}],"required":true},
            "videoType": {"dataType":"union","subSchemas":[{"ref":"VideoType"},{"dataType":"enum","enums":[null]}],"required":true},
            "isExplicit": {"dataType":"boolean","required":true},
            "counterpart": {"dataType":"union","subSchemas":[{"ref":"QueueTrack"},{"dataType":"enum","enums":[null]}],"required":true},
        },
        "additionalProperties": false,
    },
    // WARNING: This file was auto-generated with tsoa. Please do not modify it. Re-run tsoa to re-generate this file: https://github.com/lukeautry/tsoa
    "Queue": {
        "dataType": "refObject",
        "properties": {
            "chips": {"dataType":"array","array":{"dataType":"refObject","ref":"QueueChip"},"required":true},
            "playlistId": {"dataType":"union","subSchemas":[{"dataType":"string"},{"dataType":"enum","enums":[null]}],"required":true},
            "playlist": {"dataType":"union","subSchemas":[{"dataType":"string"},{"dataType":"enum","enums":[null]}],"required":true},
            "tracks": {"dataType":"array","array":{"dataType":"refObject","ref":"QueueTrack"},"required":true},
            "lyrics": {"dataType":"union","subSchemas":[{"dataType":"string"},{"dataType":"enum","enums":[null]}],"required":true},
            "related": {"dataType":"union","subSchemas":[{"dataType":"string"},{"dataType":"enum","enums":[null]}],"required":true},
            "author": {"dataType":"union","subSchemas":[{"dataType":"nestedObjectLiteral","nestedProperties":{"id":{"dataType":"union","subSchemas":[{"dataType":"string"},{"dataType":"enum","enums":[null]}],"required":true},"name":{"dataType":"union","subSchemas":[{"dataType":"string"},{"dataType":"enum","enums":[null]}],"required":true}}},{"dataType":"enum","enums":[null]}],"required":true},
            "continuation": {"dataType":"union","subSchemas":[{"dataType":"string"},{"dataType":"enum","enums":[null]}],"required":true},
            "current": {"dataType":"union","subSchemas":[{"dataType":"nestedObjectLiteral","nestedProperties":{"index":{"dataType":"double"},"playlistId":{"dataType":"string"},"videoId":{"dataType":"string","required":true}}},{"dataType":"enum","enums":[null]}],"required":true},
        },
        "additionalProperties": false,
    },
    // WARNING: This file was auto-generated with tsoa. Please do not modify it. Re-run tsoa to re-generate this file: https://github.com/lukeautry/tsoa
    "QueueGetParams": {
        "dataType": "refAlias",
        "type": {"dataType":"nestedObjectLiteral","nestedProperties":{"radio":{"dataType":"boolean","required":true},"videoId":{"dataType":"string","required":true}},"validators":{}},
    },
    // WARNING: This file was auto-generated with tsoa. Please do not modify it. Re-run tsoa to re-generate this file: https://github.com/lukeautry/tsoa
    "VideoFormat": {
        "dataType": "refObject",
        "properties": {
            "container": {"dataType":"union","subSchemas":[{"dataType":"enum","enums":["flv"]},{"dataType":"enum","enums":["3gp"]},{"dataType":"enum","enums":["mp4"]},{"dataType":"enum","enums":["webm"]},{"dataType":"enum","enums":["ts"]}],"required":true},
            "projection_type": {"dataType":"union","subSchemas":[{"dataType":"enum","enums":["rectangular"]},{"dataType":"enum","enums":["360"]},{"dataType":"enum","enums":["stereoscopic"]},{"dataType":"enum","enums":["3d"]}],"required":true},
            "mime_type": {"dataType":"string","required":true},
            "modified": {"dataType":"datetime","required":true},
            "itag": {"dataType":"double","required":true},
            "init_range": {"dataType":"union","subSchemas":[{"dataType":"nestedObjectLiteral","nestedProperties":{"start":{"dataType":"double","required":true},"end":{"dataType":"double","required":true}}},{"dataType":"enum","enums":[null]}],"required":true},
            "index_range": {"dataType":"union","subSchemas":[{"dataType":"nestedObjectLiteral","nestedProperties":{"start":{"dataType":"double","required":true},"end":{"dataType":"double","required":true}}},{"dataType":"enum","enums":[null]}],"required":true},
            "content_length": {"dataType":"union","subSchemas":[{"dataType":"double"},{"dataType":"enum","enums":[null]}],"required":true},
            "bitrate": {"dataType":"double","required":true},
            "average_bitrate": {"dataType":"union","subSchemas":[{"dataType":"double"},{"dataType":"enum","enums":[null]}],"required":true},
            "duration_ms": {"dataType":"double","required":true},
            "url": {"dataType":"string","required":true},
            "codecs": {"dataType":"string","required":true},
            "has_audio": {"dataType":"enum","enums":[true],"required":true},
            "has_video": {"dataType":"enum","enums":[true],"required":true},
            "width": {"dataType":"double","required":true},
            "height": {"dataType":"double","required":true},
            "quality_label": {"dataType":"union","subSchemas":[{"dataType":"enum","enums":["144p"]},{"dataType":"enum","enums":["144p 15fps"]},{"dataType":"enum","enums":["144p60 HDR"]},{"dataType":"enum","enums":["240p"]},{"dataType":"enum","enums":["240p60 HDR"]},{"dataType":"enum","enums":["270p"]},{"dataType":"enum","enums":["360p"]},{"dataType":"enum","enums":["360p60 HDR"]},{"dataType":"enum","enums":["480p"]},{"dataType":"enum","enums":["480p60 HDR"]},{"dataType":"enum","enums":["720p"]},{"dataType":"enum","enums":["720p60"]},{"dataType":"enum","enums":["720p60 HDR"]},{"dataType":"enum","enums":["1080p"]},{"dataType":"enum","enums":["1080p60"]},{"dataType":"enum","enums":["1080p60 HDR"]},{"dataType":"enum","enums":["1440p"]},{"dataType":"enum","enums":["1440p60"]},{"dataType":"enum","enums":["1440p60 HDR"]},{"dataType":"enum","enums":["2160p"]},{"dataType":"enum","enums":["2160p60"]},{"dataType":"enum","enums":["2160p60 HDR"]},{"dataType":"enum","enums":["4320p"]},{"dataType":"enum","enums":["4320p60"]}],"required":true},
            "fps": {"dataType":"double","required":true},
            "video_codec": {"dataType":"union","subSchemas":[{"dataType":"string"},{"dataType":"enum","enums":[null]}],"required":true},
            "video_quality": {"dataType":"union","subSchemas":[{"dataType":"enum","enums":["tiny"]},{"dataType":"enum","enums":["small"]},{"dataType":"enum","enums":["medium"]},{"dataType":"enum","enums":["large"]},{"dataType":"enum","enums":["hd720"]},{"dataType":"enum","enums":["hd1080"]},{"dataType":"enum","enums":["hd1440"]},{"dataType":"enum","enums":["hd2160"]},{"dataType":"enum","enums":["highres"]}],"required":true},
        },
        "additionalProperties": false,
    },
    // WARNING: This file was auto-generated with tsoa. Please do not modify it. Re-run tsoa to re-generate this file: https://github.com/lukeautry/tsoa
    "AudioFormat": {
        "dataType": "refObject",
        "properties": {
            "container": {"dataType":"union","subSchemas":[{"dataType":"enum","enums":["flv"]},{"dataType":"enum","enums":["3gp"]},{"dataType":"enum","enums":["mp4"]},{"dataType":"enum","enums":["webm"]},{"dataType":"enum","enums":["ts"]}],"required":true},
            "projection_type": {"dataType":"union","subSchemas":[{"dataType":"enum","enums":["rectangular"]},{"dataType":"enum","enums":["360"]},{"dataType":"enum","enums":["stereoscopic"]},{"dataType":"enum","enums":["3d"]}],"required":true},
            "mime_type": {"dataType":"string","required":true},
            "modified": {"dataType":"datetime","required":true},
            "itag": {"dataType":"double","required":true},
            "init_range": {"dataType":"union","subSchemas":[{"dataType":"nestedObjectLiteral","nestedProperties":{"start":{"dataType":"double","required":true},"end":{"dataType":"double","required":true}}},{"dataType":"enum","enums":[null]}],"required":true},
            "index_range": {"dataType":"union","subSchemas":[{"dataType":"nestedObjectLiteral","nestedProperties":{"start":{"dataType":"double","required":true},"end":{"dataType":"double","required":true}}},{"dataType":"enum","enums":[null]}],"required":true},
            "content_length": {"dataType":"union","subSchemas":[{"dataType":"double"},{"dataType":"enum","enums":[null]}],"required":true},
            "bitrate": {"dataType":"double","required":true},
            "average_bitrate": {"dataType":"union","subSchemas":[{"dataType":"double"},{"dataType":"enum","enums":[null]}],"required":true},
            "duration_ms": {"dataType":"double","required":true},
            "url": {"dataType":"string","required":true},
            "codecs": {"dataType":"string","required":true},
            "has_audio": {"dataType":"enum","enums":[true],"required":true},
            "has_video": {"dataType":"enum","enums":[false],"required":true},
            "audio_quality": {"dataType":"union","subSchemas":[{"dataType":"enum","enums":["tiny"]},{"dataType":"enum","enums":["low"]},{"dataType":"enum","enums":["medium"]},{"dataType":"enum","enums":["high"]}],"required":true},
            "channels": {"dataType":"double","required":true},
            "sample_rate": {"dataType":"double","required":true},
            "audio_codec": {"dataType":"union","subSchemas":[{"dataType":"string"},{"dataType":"enum","enums":[null]}],"required":true},
            "quality": {"dataType":"union","subSchemas":[{"dataType":"enum","enums":["tiny"]},{"dataType":"enum","enums":["small"]},{"dataType":"enum","enums":["medium"]},{"dataType":"enum","enums":["large"]}],"required":true},
        },
        "additionalProperties": false,
    },
    // WARNING: This file was auto-generated with tsoa. Please do not modify it. Re-run tsoa to re-generate this file: https://github.com/lukeautry/tsoa
    "Format": {
        "dataType": "refAlias",
        "type": {"dataType":"union","subSchemas":[{"ref":"VideoFormat"},{"ref":"AudioFormat"}],"validators":{}},
    },
    // WARNING: This file was auto-generated with tsoa. Please do not modify it. Re-run tsoa to re-generate this file: https://github.com/lukeautry/tsoa
    "VideoDetails": {
        "dataType": "refObject",
        "properties": {
            "videoId": {"dataType":"string","required":true},
            "title": {"dataType":"string","required":true},
            "lengthSeconds": {"dataType":"double","required":true},
            "channelId": {"dataType":"string","required":true},
            "isOwnerViewing": {"dataType":"boolean","required":true},
            "isCrawlable": {"dataType":"boolean","required":true},
            "thumbnail": {"dataType":"nestedObjectLiteral","nestedProperties":{"thumbnails":{"dataType":"array","array":{"dataType":"refObject","ref":"Thumbnail"},"required":true}},"required":true},
            "allowRatings": {"dataType":"enum","enums":[true],"required":true},
            "viewCount": {"dataType":"double","required":true},
            "author": {"dataType":"string","required":true},
            "isPrivate": {"dataType":"boolean","required":true},
            "isUnpluggedCorpus": {"dataType":"boolean","required":true},
            "musicVideoType": {"ref":"VideoType","required":true},
            "isLiveContent": {"dataType":"boolean","required":true},
        },
        "additionalProperties": false,
    },
    // WARNING: This file was auto-generated with tsoa. Please do not modify it. Re-run tsoa to re-generate this file: https://github.com/lukeautry/tsoa
    "Caption": {
        "dataType": "refObject",
        "properties": {
            "url": {"dataType":"string","required":true},
            "name": {"dataType":"string","required":true},
            "vssId": {"dataType":"string","required":true},
            "lang": {"dataType":"string","required":true},
            "translatable": {"dataType":"boolean","required":true},
        },
        "additionalProperties": false,
    },
    // WARNING: This file was auto-generated with tsoa. Please do not modify it. Re-run tsoa to re-generate this file: https://github.com/lukeautry/tsoa
    "Song": {
        "dataType": "refObject",
        "properties": {
            "formats": {"dataType":"array","array":{"dataType":"refAlias","ref":"Format"},"required":true},
            "adaptive_formats": {"dataType":"array","array":{"dataType":"refAlias","ref":"Format"},"required":true},
            "expires": {"dataType":"datetime","required":true},
            "videoDetails": {"ref":"VideoDetails","required":true},
            "playerConfig": {"dataType":"any","required":true},
            "playbackTracking": {"dataType":"any","required":true},
            "videostatsPlaybackUrl": {"dataType":"string","required":true},
            "captions": {"dataType":"array","array":{"dataType":"refObject","ref":"Caption"},"required":true},
            "hlsManifestUrl": {"dataType":"union","subSchemas":[{"dataType":"string"},{"dataType":"enum","enums":[null]}],"required":true},
            "aspectRatio": {"dataType":"double","required":true},
            "serverAbrStreamingUrl": {"dataType":"string","required":true},
        },
        "additionalProperties": false,
    },
    // WARNING: This file was auto-generated with tsoa. Please do not modify it. Re-run tsoa to re-generate this file: https://github.com/lukeautry/tsoa
    "SongGetParams": {
        "dataType": "refAlias",
        "type": {"dataType":"nestedObjectLiteral","nestedProperties":{"videoId":{"dataType":"string","required":true}},"validators":{}},
    },
    // WARNING: This file was auto-generated with tsoa. Please do not modify it. Re-run tsoa to re-generate this file: https://github.com/lukeautry/tsoa
};
const validationService = new ValidationService(models);

// WARNING: This file was auto-generated with tsoa. Please do not modify it. Re-run tsoa to re-generate this file: https://github.com/lukeautry/tsoa

export function RegisterRoutes(app: Router) {
    // ###########################################################################################################
    //  NOTE: If you do not see routes for all of your controllers in this file, then you might not have informed tsoa of where to look
    //      Please look into the "controllerPathGlobs" config option described in the readme: https://github.com/lukeautry/tsoa
    // ###########################################################################################################
        app.post('/playlist',
            ...(fetchMiddlewares<RequestHandler>(PlaylistController)),
            ...(fetchMiddlewares<RequestHandler>(PlaylistController.prototype.getPlaylist)),

            function PlaylistController_getPlaylist(request: any, response: any, next: any) {
            const args = {
                    body: {"in":"body","name":"body","required":true,"ref":"PlaylistGetParams"},
            };

            // WARNING: This file was auto-generated with tsoa. Please do not modify it. Re-run tsoa to re-generate this file: https://github.com/lukeautry/tsoa

            let validatedArgs: any[] = [];
            try {
                validatedArgs = getValidatedArgs(args, request, response);

                const controller = new PlaylistController();


              const promise = controller.getPlaylist.apply(controller, validatedArgs as any);
              promiseHandler(controller, promise, response, undefined, next);
            } catch (err) {
                return next(err);
            }
        });
        // WARNING: This file was auto-generated with tsoa. Please do not modify it. Re-run tsoa to re-generate this file: https://github.com/lukeautry/tsoa
        app.post('/album',
            ...(fetchMiddlewares<RequestHandler>(AlbumController)),
            ...(fetchMiddlewares<RequestHandler>(AlbumController.prototype.getAlbum)),

            function AlbumController_getAlbum(request: any, response: any, next: any) {
            const args = {
                    body: {"in":"body","name":"body","required":true,"ref":"AlbumGetParams"},
            };

            // WARNING: This file was auto-generated with tsoa. Please do not modify it. Re-run tsoa to re-generate this file: https://github.com/lukeautry/tsoa

            let validatedArgs: any[] = [];
            try {
                validatedArgs = getValidatedArgs(args, request, response);

                const controller = new AlbumController();


              const promise = controller.getAlbum.apply(controller, validatedArgs as any);
              promiseHandler(controller, promise, response, undefined, next);
            } catch (err) {
                return next(err);
            }
        });
        // WARNING: This file was auto-generated with tsoa. Please do not modify it. Re-run tsoa to re-generate this file: https://github.com/lukeautry/tsoa
        app.post('/albumByPlaylistId',
            ...(fetchMiddlewares<RequestHandler>(AlbumByPlaylistIdController)),
            ...(fetchMiddlewares<RequestHandler>(AlbumByPlaylistIdController.prototype.getAlbumByPlaylistId)),

            function AlbumByPlaylistIdController_getAlbumByPlaylistId(request: any, response: any, next: any) {
            const args = {
                    body: {"in":"body","name":"body","required":true,"ref":"AlbumGetByPlaylistIdParams"},
            };

            // WARNING: This file was auto-generated with tsoa. Please do not modify it. Re-run tsoa to re-generate this file: https://github.com/lukeautry/tsoa

            let validatedArgs: any[] = [];
            try {
                validatedArgs = getValidatedArgs(args, request, response);

                const controller = new AlbumByPlaylistIdController();


              const promise = controller.getAlbumByPlaylistId.apply(controller, validatedArgs as any);
              promiseHandler(controller, promise, response, undefined, next);
            } catch (err) {
                return next(err);
            }
        });
        // WARNING: This file was auto-generated with tsoa. Please do not modify it. Re-run tsoa to re-generate this file: https://github.com/lukeautry/tsoa
        app.post('/queue',
            ...(fetchMiddlewares<RequestHandler>(QueueController)),
            ...(fetchMiddlewares<RequestHandler>(QueueController.prototype.getQueue)),

            function QueueController_getQueue(request: any, response: any, next: any) {
            const args = {
                    body: {"in":"body","name":"body","required":true,"ref":"QueueGetParams"},
            };

            // WARNING: This file was auto-generated with tsoa. Please do not modify it. Re-run tsoa to re-generate this file: https://github.com/lukeautry/tsoa

            let validatedArgs: any[] = [];
            try {
                validatedArgs = getValidatedArgs(args, request, response);

                const controller = new QueueController();


              const promise = controller.getQueue.apply(controller, validatedArgs as any);
              promiseHandler(controller, promise, response, undefined, next);
            } catch (err) {
                return next(err);
            }
        });
        // WARNING: This file was auto-generated with tsoa. Please do not modify it. Re-run tsoa to re-generate this file: https://github.com/lukeautry/tsoa
        app.post('/song',
            ...(fetchMiddlewares<RequestHandler>(SongController)),
            ...(fetchMiddlewares<RequestHandler>(SongController.prototype.getSong)),

            function SongController_getSong(request: any, response: any, next: any) {
            const args = {
                    body: {"in":"body","name":"body","required":true,"ref":"SongGetParams"},
            };

            // WARNING: This file was auto-generated with tsoa. Please do not modify it. Re-run tsoa to re-generate this file: https://github.com/lukeautry/tsoa

            let validatedArgs: any[] = [];
            try {
                validatedArgs = getValidatedArgs(args, request, response);

                const controller = new SongController();


              const promise = controller.getSong.apply(controller, validatedArgs as any);
              promiseHandler(controller, promise, response, undefined, next);
            } catch (err) {
                return next(err);
            }
        });
        // WARNING: This file was auto-generated with tsoa. Please do not modify it. Re-run tsoa to re-generate this file: https://github.com/lukeautry/tsoa

    // WARNING: This file was auto-generated with tsoa. Please do not modify it. Re-run tsoa to re-generate this file: https://github.com/lukeautry/tsoa


    // WARNING: This file was auto-generated with tsoa. Please do not modify it. Re-run tsoa to re-generate this file: https://github.com/lukeautry/tsoa

    function isController(object: any): object is Controller {
        return 'getHeaders' in object && 'getStatus' in object && 'setStatus' in object;
    }

    function promiseHandler(controllerObj: any, promise: any, response: any, successStatus: any, next: any) {
        return Promise.resolve(promise)
            .then((data: any) => {
                let statusCode = successStatus;
                let headers;
                if (isController(controllerObj)) {
                    headers = controllerObj.getHeaders();
                    statusCode = controllerObj.getStatus() || statusCode;
                }

                // WARNING: This file was auto-generated with tsoa. Please do not modify it. Re-run tsoa to re-generate this file: https://github.com/lukeautry/tsoa

                returnHandler(response, statusCode, data, headers)
            })
            .catch((error: any) => next(error));
    }

    // WARNING: This file was auto-generated with tsoa. Please do not modify it. Re-run tsoa to re-generate this file: https://github.com/lukeautry/tsoa

    function returnHandler(response: any, statusCode?: number, data?: any, headers: any = {}) {
        if (response.headersSent) {
            return;
        }
        Object.keys(headers).forEach((name: string) => {
            response.set(name, headers[name]);
        });
        if (data && typeof data.pipe === 'function' && data.readable && typeof data._read === 'function') {
            response.status(statusCode || 200)
            data.pipe(response);
        } else if (data !== null && data !== undefined) {
            response.status(statusCode || 200).json(data);
        } else {
            response.status(statusCode || 204).end();
        }
    }

    // WARNING: This file was auto-generated with tsoa. Please do not modify it. Re-run tsoa to re-generate this file: https://github.com/lukeautry/tsoa

    function responder(response: any): TsoaResponse<HttpStatusCodeLiteral, unknown>  {
        return function(status, data, headers) {
            returnHandler(response, status, data, headers);
        };
    };

    // WARNING: This file was auto-generated with tsoa. Please do not modify it. Re-run tsoa to re-generate this file: https://github.com/lukeautry/tsoa

    function getValidatedArgs(args: any, request: any, response: any): any[] {
        const fieldErrors: FieldErrors  = {};
        const values = Object.keys(args).map((key) => {
            const name = args[key].name;
            switch (args[key].in) {
                case 'request':
                    return request;
                case 'query':
                    return validationService.ValidateParam(args[key], request.query[name], name, fieldErrors, undefined, {"noImplicitAdditionalProperties":"throw-on-extras"});
                case 'queries':
                    return validationService.ValidateParam(args[key], request.query, name, fieldErrors, undefined, {"noImplicitAdditionalProperties":"throw-on-extras"});
                case 'path':
                    return validationService.ValidateParam(args[key], request.params[name], name, fieldErrors, undefined, {"noImplicitAdditionalProperties":"throw-on-extras"});
                case 'header':
                    return validationService.ValidateParam(args[key], request.header(name), name, fieldErrors, undefined, {"noImplicitAdditionalProperties":"throw-on-extras"});
                case 'body':
                    return validationService.ValidateParam(args[key], request.body, name, fieldErrors, undefined, {"noImplicitAdditionalProperties":"throw-on-extras"});
                case 'body-prop':
                    return validationService.ValidateParam(args[key], request.body[name], name, fieldErrors, 'body.', {"noImplicitAdditionalProperties":"throw-on-extras"});
                case 'formData':
                    if (args[key].dataType === 'file') {
                        return validationService.ValidateParam(args[key], request.file, name, fieldErrors, undefined, {"noImplicitAdditionalProperties":"throw-on-extras"});
                    } else if (args[key].dataType === 'array' && args[key].array.dataType === 'file') {
                        return validationService.ValidateParam(args[key], request.files, name, fieldErrors, undefined, {"noImplicitAdditionalProperties":"throw-on-extras"});
                    } else {
                        return validationService.ValidateParam(args[key], request.body[name], name, fieldErrors, undefined, {"noImplicitAdditionalProperties":"throw-on-extras"});
                    }
                case 'res':
                    return responder(response);
            }
        });

        if (Object.keys(fieldErrors).length > 0) {
            throw new ValidateError(fieldErrors, '');
        }
        return values;
    }

    // WARNING: This file was auto-generated with tsoa. Please do not modify it. Re-run tsoa to re-generate this file: https://github.com/lukeautry/tsoa
}

// WARNING: This file was auto-generated with tsoa. Please do not modify it. Re-run tsoa to re-generate this file: https://github.com/lukeautry/tsoa
