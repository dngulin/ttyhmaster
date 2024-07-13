<?php
require_once __DIR__ . DIRECTORY_SEPARATOR . 'functions.php';

function set_mojang(string $user, bool $flag): bool
{
    $code = ttyh_master_update_player_is_mojang($user, $flag);
    return $code == 200;
}

if ($section === 'minecraft') {
    $isMojang = $_POST['isMojang'] ? true : false;
    if (!set_mojang($user['username'], $isMojang)) {
        $errors[] = 'Что-то пошло не так. Пиши багрепорт.';
    }
    if (empty($errors)) {
        $forum_flash->add_info($lang_profile['Profile redirect']);
        redirect(forum_link('profile.php?section=minecraft&amp;id=$1', $id), $lang_profile['Profile redirect']);
    }
}
