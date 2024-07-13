<?php
require_once __DIR__ . DIRECTORY_SEPARATOR . 'init.php';

function set_mojang(string $user, bool $flag): bool
{
    $conn = dbconn();

    $stmt = $conn->prepare('UPDATE players SET isMojang = ? WHERE player = ?');
    $stmt->bindParam(1, $flag, PDO::PARAM_BOOL);
    $stmt->bindParam(2, $user, PDO::PARAM_STR);
    if (!$stmt->execute()) {
        return false;
    }
    return true;
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
