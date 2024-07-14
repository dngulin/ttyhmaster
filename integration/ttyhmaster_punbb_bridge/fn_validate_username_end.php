<?php
if (file_exists(__DIR__ . '/lang/' . $forum_user['language'] . '/profile_ttyhmaster.php')) {
    require_once __DIR__ . '/lang/' . $forum_user['language'] . '/profile_ttyhmaster.php';
} else {
    require_once __DIR__ . '/lang/English/profile_ttyhmaster.php';
}

if (preg_match('/[^a-zA-Z0-9_]/', $username)) {
    $errors[] = $lang_profile_ttyhmaster['Bad username'];
}