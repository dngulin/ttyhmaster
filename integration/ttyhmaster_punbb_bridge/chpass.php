<?php
require_once __DIR__ . DIRECTORY_SEPARATOR . 'functions.php';
ttyhbridge_update_player_password($user['username'], $new_password_hash, $user['salt']);