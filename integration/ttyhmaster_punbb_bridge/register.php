<?php
require_once __DIR__ . DIRECTORY_SEPARATOR . 'functions.php';

$query = [
    'SELECT' => 'password',
    'FROM' => 'users',
    'WHERE' => "username = '{$forum_db->escape($username)}'",
];
$result = $forum_db->query_build($query) or exit(21);
$password = $forum_db->fetch_assoc($result)['password'];

$code = ttyh_master_create_player($username, $password, $salt);
if ($code != 200) {
    error_log("Failed to create player `{$username}`: response code {$code}");
}