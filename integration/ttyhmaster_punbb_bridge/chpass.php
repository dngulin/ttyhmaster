<?php
require_once __DIR__ . DIRECTORY_SEPARATOR . 'init.php';

$conn = dbconn();

$stmt = $conn->prepare('UPDATE players SET password = ?, salt = ? WHERE player = ?');
$stmt->execute([$new_password_hash, $user['salt'], $user['username']]);
error_log("CHPASS: {$new_password_hash} {$user['salt']} {$user['username']}");
