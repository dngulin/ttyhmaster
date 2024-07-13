<?php
const TTYHBRIDGE_HOST = 'http://example.com';
const TTYHBRIDGE_TOKEN = 'example-token';

function ttyhbridge_get_response_code($http_response_header): int
{
    if (is_array($http_response_header)) {
        $parts = explode(' ', $http_response_header[0]);
        if (count($parts) > 1) //HTTP/1.1 <code> <text>
            return intval($parts[1]);
    }

    return 0;
}

// Returns [ 'code' => 200, 'payload' => [ ... ] ] or [ 'code' => int ]
function ttyhbridge_get($endpoint): array
{
    $url = TTYHBRIDGE_HOST . '/' . $endpoint;
    $auth_header = 'Authorization: Bearer ' . TTYHBRIDGE_TOKEN;

    $options = [
        'http' => [
            'method' => 'GET',
            'header' => $auth_header
        ],
    ];

    $response = file_get_contents($url, false, stream_context_create($options));
    $code = ttyhbridge_get_response_code($http_response_header);

    if ($code != 200) {
        return ['code' => $code];
    }

    return ['code' => $code, 'payload' => json_decode($response)];
}

function ttyhbridge_post($endpoint, $payload): int
{
    $url = TTYHBRIDGE_HOST . '/' . $endpoint;
    $auth_header = 'Authorization: Bearer ' . TTYHBRIDGE_TOKEN;

    $options = [
        'http' => [
            'method' => 'POST',
            'header' => $auth_header,
            'content' => json_encode($payload),
        ],
    ];

    file_get_contents($url, false, stream_context_create($options));
    return ttyhbridge_get_response_code($http_response_header);
}

function ttyhbridge_create_player($name, $pwd_hash, $pwd_salt): int
{
    $payload = [
        'player_name' => $name,
        'password' => [
            'hash' => $pwd_hash,
            'salt' => $pwd_salt,
        ]
    ];
    return ttyhbridge_post('ttyh/player/create', $payload);
}

function ttyhbridge_update_player($name, $payload): int
{
    $endpoint = 'ttyh/player/' . urlencode($name) . '/update';
    return ttyhbridge_post($endpoint, $payload);
}

function ttyhbridge_update_player_name($name, $new_name): int
{
    $payload = [
        'player_name' => $new_name
    ];
    return ttyhbridge_update_player($name, $payload);
}

function ttyhbridge_update_player_password($name, $pwd_hash, $pwd_salt): int
{
    $payload = [
        'password' => [
            'hash' => $pwd_hash,
            'salt' => $pwd_salt,
        ]
    ];
    return ttyhbridge_update_player($name, $payload);
}

function ttyhbridge_update_player_is_mojang($name, $is_mojang): int
{
    $payload = [
        'is_mojang' => $is_mojang
    ];

    return ttyhbridge_update_player($name, $payload);
}

// returns [ 'code' => 200, 'payload' => [ 'player_id' => string, 'is_mojang' => bool ] ] or [ 'code' => int ]
function ttyhbridge_query_player($name): array
{
    return ttyhbridge_get('ttyh/player/' . urlencode($name));
}
