<?php
require_once __DIR__ . DIRECTORY_SEPARATOR . 'functions.php';

if (file_exists(__DIR__ . '/lang/' . $forum_user['language'] . '/profile_ttyhmaster.php')) {
    require_once __DIR__ . '/lang/' . $forum_user['language'] . '/profile_ttyhmaster.php';
} else {
    require_once __DIR__ . '/lang/English/profile_ttyhmaster.php';
}

if ($section === 'minecraft') {

    switch ($_POST['form_type']) {
        case 'create':
            $ttyh_response_code = ttyh_master_create_player($user['username'], $user['password'], $user['salt']);
            if ($ttyh_response_code != 200) {
                $errors[] = $lang_profile_ttyhmaster['Failed to link account: '] . $ttyh_response_code;
            }
            break;

        case 'update':
            $ttyh_is_mojang = $_POST['form']['is_mojang'] == 1;
            $ttyh_response_code = ttyh_master_update_player_is_mojang($user['username'], $ttyh_is_mojang);
            if ($ttyh_response_code != 200) {
                $errors[] = $lang_profile_ttyhmaster['Failed to update auth settings: '] . $ttyh_response_code;
            }
            break;

        default:
            $errors[] = 'Unknown form type: ' . $_POST['form_type'];
            break;
    }

    if (empty($errors)) {
        $forum_flash->add_info($lang_profile['Profile redirect']);
        redirect(forum_link('profile.php?section=minecraft&amp;id=$1', $id), $lang_profile['Profile redirect']);
    }
}
