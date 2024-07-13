<?php
require_once __DIR__ . DIRECTORY_SEPARATOR . 'init.php';

$query = [
    'SELECT' => 'password',
    'FROM'   => 'users',
    'WHERE'  => "username = '{$forum_db->escape($username)}'",
];
$result = $forum_db->query_build($query) or exit(21);
$password = $forum_db->fetch_assoc($result)['password'];

$conn = dbconn();
$stmt = $conn->prepare(
    'INSERT INTO players (player, password, salt, clientToken, accessToken, registered_at) ' .
    'VALUES (?, ?, ?, ?, ?, CURRENT_TIMESTAMP)'
);
$stmt->execute([$username, $password, $salt, generate_uuid(), generate_uuid()]) or exit(42);

error_log("REGISTERED: {$username}, {$password}, {$salt}");
