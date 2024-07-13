<?php
require_once __DIR__ . DIRECTORY_SEPARATOR . 'functions.php';

$code = ttyh_master_create_player($username, $password_hash, $salt);
if ($code != 200) {
    error_log("Failed to link player `{$username}`: response code {$code}");
}