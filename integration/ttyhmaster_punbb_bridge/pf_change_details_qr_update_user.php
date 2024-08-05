<?php
require_once __DIR__ . DIRECTORY_SEPARATOR . 'functions.php';

if ($username_updated) {
    $_response_code = ttyh_master_update_player_name($old_username, $form['username']);
    if ($_response_code != 200) {
        error_log("Failed to update user name on master server ({$_response_code})");
    }
}
