<?php
require_once __DIR__ . DIRECTORY_SEPARATOR . 'functions.php';

$_response_code = ttyh_master_update_player_password($user['username'], $new_password_hash, $user['salt']);
if ($_response_code != 200) {
    error("Failed to update user password on master server ({$_response_code})", __FILE__, __LINE__);
}