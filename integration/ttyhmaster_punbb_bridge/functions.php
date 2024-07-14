<?php
const TTYH_MASTER_HOST = 'http://example.com';
const TTYH_MASTER_API_TOKEN = 'example-token';
const TTYH_MASTER_LANG_DIR = __DIR__ . DIRECTORY_SEPARATOR . 'lang';

function _ttyh_master_get_response_code($http_response_header): int
{
    if (is_array($http_response_header)) {
        $parts = explode(' ', $http_response_header[0]);
        if (count($parts) > 1) //HTTP/1.1 <code> <text>
            return intval($parts[1]);
    }

    return 0;
}

// Returns [ 'code' => 200, 'payload' => [ ... ] ] or [ 'code' => int ]
function _ttyh_master_get($endpoint): array
{
    $url = TTYH_MASTER_HOST . '/' . $endpoint;
    $auth_header = 'Authorization: Bearer ' . TTYH_MASTER_API_TOKEN;

    $options = [
        'http' => [
            'method' => 'GET',
            'header' => $auth_header
        ],
    ];

    $response = file_get_contents($url, false, stream_context_create($options));
    $code = _ttyh_master_get_response_code($http_response_header);

    if ($code != 200) {
        return ['code' => $code];
    }

    return ['code' => $code, 'payload' => json_decode($response)];
}

function _ttyh_master_post($endpoint, $payload): int
{
    $url = TTYH_MASTER_HOST . '/' . $endpoint;
    $auth_header = 'Authorization: Bearer ' . TTYH_MASTER_API_TOKEN;

    $options = [
        'http' => [
            'method' => 'POST',
            'header' => $auth_header,
            'content' => json_encode($payload),
        ],
    ];

    file_get_contents($url, false, stream_context_create($options));
    return _ttyh_master_get_response_code($http_response_header);
}

function ttyh_master_create_player($name, $pwd_hash, $pwd_salt): int
{
    $payload = [
        'player_name' => $name,
        'password' => [
            'hash' => $pwd_hash,
            'salt' => $pwd_salt,
        ]
    ];
    return _ttyh_master_post('ttyh/player/create', $payload);
}

function _ttyh_master_update_player($name, $payload): int
{
    $endpoint = 'ttyh/player/' . urlencode($name) . '/update';
    return _ttyh_master_post($endpoint, $payload);
}

function ttyh_master_update_player_name($name, $new_name): int
{
    $payload = [
        'player_name' => $new_name
    ];
    return _ttyh_master_update_player($name, $payload);
}

function ttyh_master_update_player_password($name, $pwd_hash, $pwd_salt): int
{
    $payload = [
        'password' => [
            'hash' => $pwd_hash,
            'salt' => $pwd_salt,
        ]
    ];
    return _ttyh_master_update_player($name, $payload);
}

function ttyh_master_update_player_is_mojang($name, $is_mojang): int
{
    $payload = [
        'is_mojang' => $is_mojang
    ];

    return _ttyh_master_update_player($name, $payload);
}

// returns [ 'code' => 200, 'payload' => [ 'player_id' => string, 'is_mojang' => bool ] ] or [ 'code' => int ]
function ttyh_master_query_player($name): array
{
    return _ttyh_master_get('ttyh/player/' . urlencode($name));
}
