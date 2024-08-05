<?php
require_once __DIR__ . DIRECTORY_SEPARATOR . 'functions.php';

$_response_code = ttyh_master_create_player($username, $password_hash, $salt);
if ($_response_code != 200) {
    error_log("Failed to create a new user `{$username}` on master server ({$_response_code})");
}