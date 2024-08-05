<?php
require_once __DIR__ . '/functions.php';

if (file_exists(__DIR__ . '/lang/' . $forum_user['language'] . '/profile_ttyhmaster.php')) {
    require_once __DIR__ . '/lang/' . $forum_user['language'] . '/profile_ttyhmaster.php';
} else {
    require_once __DIR__ . '/lang/English/profile_ttyhmaster.php';
}

if ($section === 'minecraft') {
    // Setup breadcrumbs
    $forum_page['crumbs'] = [
        [$forum_config['o_board_title'], forum_link($forum_url['index'])],
        [sprintf($lang_profile['Users profile'], $user['username']), forum_link($forum_url['user'], $id)],
        'Minecraft',
    ];
    // Setup the form
    $forum_page['form_action'] = forum_link('profile.php?section=minecraft&amp;id=$1', $id);
    $forum_page['hidden_fields'] = [
        'form_sent' => '<input type="hidden" name="form_sent" value="1" />',
        'csrf_token' => '<input type="hidden" name="csrf_token" value="' . generate_form_token($forum_page['form_action']) . '" />'
    ];
    // Setup form information
    define('FORUM_PAGE', 'profile-minecraft');
    require FORUM_ROOT . 'header.php';

    $ttyhmaster_response = ttyh_master_query_player($user['username']);
    switch ($ttyhmaster_response['code']) {
        case 200:
            $forum_page['hidden_fields']['form_type'] = '<input type="hidden" name="form_type" value="update" />';
            break;
        case 404:
            $forum_page['hidden_fields']['form_type'] = '<input type="hidden" name="form_type" value="create" />';
            break;
        default:
            $errors[] = $lang_profile_ttyhmaster['Failed to query player: '] . $ttyhmaster_response['code'];
            break;
    }


    // START SUBST - <!-- forum_main -->
    ob_start();

    echo <<<END
<div class="main-subhead">
    <h2 class="hn"><span>{$lang_profile_ttyhmaster['Title']}</span></h2>
</div>
<div class="main-content main-frm">

END;

    // If there were any errors, show them
    if (!empty($errors)) {
        $forum_page['errors'] = array();
        foreach ($errors as $cur_error) {
            $forum_page['errors'][] = '<li class="warn"><span>' . $cur_error . '</span></li>';
        }
        $errors_formatted = implode("\n\t\t\t", $forum_page['errors']) . "\n";

        echo <<<END
<div class="ct-box error-box">
    <h2 class="warn hn">{$lang_profile['Profile update errors']}</h2>
    <ul class="error-list">
        {$errors_formatted}
    </ul>
</div>

END;
    }

    $hidden_fields_formatted = implode("\n\t\t\t\t", $forum_page['hidden_fields']) . "\n";

    if ($ttyhmaster_response['code'] == 200) {
        $is_mojang_checked_attr = $ttyhmaster_response['payload']['is_mojang'] ? 'checked="checked"' : '';

        echo <<<END
<form class="frm-form" method="post" accept-charset="utf-8" action="{$forum_page['form_action']}" enctype="multipart/form-data">
    <div class="hidden">
        {$hidden_fields_formatted}
    </div>
    <fieldset class="frm-group group1">
        <legend class="group-legend"><strong>Minecraft Settings</strong></legend>
        <div class="ct-set set1">
            <div class="ct-box">
                <h3 class="hn ct-legend">UUID</h3>
                <p>{$ttyhmaster_response['payload']['player_id']}</p>
            </div>
        </div>
        <div class="sf-set set2">
            <div class="sf-box checkbox">
                <span class="fld-input"><input type="checkbox" id="fld2" name="form[is_mojang]" value="1" {$is_mojang_checked_attr}/></span>
                <label for="fld2">{$lang_profile_ttyhmaster['Mojang auth']}</label>
            </div>
        </div>
    </fieldset>
    <div class="frm-buttons">
        <span class="submit primary"><input type="submit" name="update" value="{$lang_profile['Update profile']}" /></span>
    </div>
</form>

END;
    } elseif ($ttyhmaster_response['code'] == 404) {
        echo <<<END
<form class="frm-form" method="post" accept-charset="utf-8" action="{$forum_page['form_action']}" enctype="multipart/form-data">
    <div class="hidden">
        {$hidden_fields_formatted}
    </div>
    <div class="ct-box info-box">
        <p>{$lang_profile_ttyhmaster['Account is not linked']}</p>
    </div>
    <div class="frm-buttons">
        <span class="submit primary"><input type="submit" name="update" value="{$lang_profile_ttyhmaster['Link account']}" /></span>
    </div>
</form>

END;
    }

    echo '</div>'; // <div class="main-content main-frm">

    $tpl_temp = forum_trim(ob_get_contents());
    $tpl_main = str_replace('<!-- forum_main -->', $tpl_temp, $tpl_main);
    ob_end_clean();
    // END SUBST - <!-- forum_main -->

    require FORUM_ROOT . 'footer.php';
}
