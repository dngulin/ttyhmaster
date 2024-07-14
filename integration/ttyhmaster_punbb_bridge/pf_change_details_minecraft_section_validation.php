<?php
require_once __DIR__ . DIRECTORY_SEPARATOR . 'functions.php';

if (file_exists(__DIR__ . '/lang/' . $forum_user['language'] . '/profile_ttyhmaster.php')) {
    require_once __DIR__ . '/lang/' . $forum_user['language'] . '/profile_ttyhmaster.php';
} else {
    require_once __DIR__ . '/lang/English/profile_ttyhmaster.php';
}

function set_mojang(string $user, bool $flag): bool
{
    $code = ttyh_master_update_player_is_mojang($user, $flag);
    return $code == 200;
}

if ($section === 'minecraft') {
    $isMojang = $_POST['isMojang'] ? true : false;
    if (!set_mojang($user['username'], $isMojang)) {
        $errors[] = $lang_profile_ttyhmaster['Failed To Update Auth Settings'];
    }
    if (empty($errors)) {
        $forum_flash->add_info($lang_profile['Profile redirect']);
        redirect(forum_link('profile.php?section=minecraft&amp;id=$1', $id), $lang_profile['Profile redirect']);
    }
}
