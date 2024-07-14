<?php
require_once __DIR__ . DIRECTORY_SEPARATOR . 'functions.php';

if (file_exists(TTYH_MASTER_LANG_DIR . '/' . $forum_user['language'] . '/pf_section.php')) {
    require_once TTYH_MASTER_LANG_DIR . '/' . $forum_user['language'] . '/pf_section.php';
} else {
    require_once TTYH_MASTER_LANG_DIR . '/English/pf_section.php';
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
