<?php

if (!defined('TTYH_MASTER_HOST'))
    define('TTYH_MASTER_HOST', 'http://localhost');

if (!defined('TTYH_MASTER_API_TOKEN'))
    define('TTYH_MASTER_API_TOKEN', 'ttyhmaster-api-token');

const TTYH_MASTER_HEADER = 'Authorization: Bearer ' . TTYH_MASTER_API_TOKEN;

function _ttyh_master_get_response_code($http_response_header): int
{
    if (is_array($http_response_header)) {
        $parts = explode(' ', $http_response_header[0]);
        if (count($parts) > 1) // HTTP/1.1 <code> <text>
            return intval($parts[1]);
    }

    return 0;
}

// Returns [ 'code' => 200, 'payload' => [ ... ] ] or [ 'code' => int ]
function _ttyh_master_get($endpoint): array
{
    $options = [
        'http' => [
            'method' => 'GET',
            'header' => TTYH_MASTER_HEADER
        ],
    ];

    try {
        $response = file_get_contents(TTYH_MASTER_HOST . $endpoint, false, stream_context_create($options));
    } catch (Exception $e) {
        error_log("Failed to perform a GET request to {$endpoint}: " . $e->getMessage());
        return ['code' => -1];
    }

    $code = _ttyh_master_get_response_code($http_response_header);

    if ($code != 200) {
        return ['code' => $code];
    }

    return ['code' => $code, 'payload' => json_decode($response)];
}

function _ttyh_master_post($endpoint, $payload): int
{
    $options = [
        'http' => [
            'method' => 'POST',
            'header' => [
                'Content-Type: application/json',
                TTYH_MASTER_HEADER,
            ],
            'content' => json_encode($payload),
        ],
    ];

    try {
        file_get_contents(TTYH_MASTER_HOST . $endpoint, false, stream_context_create($options));
    } catch (Exception $e) {
        error_log("Failed to perform a POST request to $endpoint: " . $e->getMessage());
        return -1;
    }

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
    return _ttyh_master_post('/ttyh/player', $payload);
}

function _ttyh_master_update_player($name, $payload): int
{
    return _ttyh_master_post('/ttyh/player/' . urlencode($name), $payload);
}

function ttyh_master_update_player_name($name, $new_name): int
{
    return _ttyh_master_update_player($name, ['player_name' => $new_name]);
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
    return _ttyh_master_update_player($name, ['is_mojang' => $is_mojang]);
}

// returns [ 'code' => 200, 'payload' => [ 'player_id' => string, 'is_mojang' => bool ] ] or [ 'code' => int ]
function ttyh_master_query_player($name): array
{
    return _ttyh_master_get('/ttyh/player/' . urlencode($name));
}
