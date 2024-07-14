<?php
require_once __DIR__ . '/functions.php';

if (file_exists(__DIR__ . '/lang/' . $forum_user['language'] . '/profile_ttyhmaster.php')) {
    require_once __DIR__ . '/lang/' . $forum_user['language'] . '/profile_ttyhmaster.php';
} else {
    require_once __DIR__ . '/lang/English/profile_ttyhmaster.php';
}

function is_mojang(string $user): bool
{
    $result = ttyh_master_query_player($user);
    if ($result['code'] != 200) {
        return false;
    }
    return $result['payload']['is_mojang'];
}

if ($section === 'minecraft') {
    // Setup breadcrumbs
    $forum_page['crumbs'] = [
        [$forum_config['o_board_title'], forum_link($forum_url['index'])],
        [sprintf($lang_profile['Users profile'], $user['username']), forum_link($forum_url['user'], $id)],
        'Minecraft',
    ];
    // Setup the form
    $forum_page['group_count'] = $forum_page['item_count'] = $forum_page['fld_count'] = 0;
    $forum_page['form_action'] = forum_link('profile.php?section=minecraft&amp;id=$1', $id);
    $forum_page['hidden_fields'] = [
        'form_sent' => '<input type="hidden" name="form_sent" value="1" />',
        'csrf_token' => '<input type="hidden" name="csrf_token" value="' . generate_form_token($forum_page['form_action']) . '" />'
    ];
    // Setup form information
    $forum_page['frm_info'] = [];
    define('FORUM_PAGE', 'profile-minecraft');
    require FORUM_ROOT . 'header.php';
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

    $forum_page['frm_info'][] = $lang_profile_ttyhmaster['Description'];
    foreach ($lang_profile_ttyhmaster['Description items'] as $frm_info_desc_item) {
        $forum_page['frm_info'][] = "<li><span>{$frm_info_desc_item}</span></li>";
    }

    ++$forum_page['item_count'];
    ++$forum_page['fld_count'];
    $hidden_fields_formatted = implode("\n\t\t\t\t", $forum_page['hidden_fields']) . "\n";
    $frm_info_formatted = implode("\n\t\t\t\t\t", $forum_page['frm_info']) . "\n";
    $checked_insertion = is_mojang($user['username']) ? ' checked="checked"' : '';

    echo <<<END
	<form class="frm-form" method="post" accept-charset="utf-8" action="{$forum_page['form_action']}" enctype="multipart/form-data">
		<div class="hidden">
			{$hidden_fields_formatted}
		</div>
		<div class="ct-box info-box">
			<ul class="info-list">
				{$frm_info_formatted}
			</ul>
		</div>
		<fieldset class="mf-set set{$forum_page['item_count']}">
			<legend><span></span></legend>
			<div class="mf-box"><div class="mf-item">
				<span class="fld-input"><input type="checkbox" id="fld{$forum_page['fld_count']}" name="isMojang" value="1"$checked_insertion /></span>
				<label for="fld{$forum_page['fld_count']}">{$lang_profile_ttyhmaster['Mojang auth']}</label>
			</div></div>
		</fieldset>
		<div class="frm-buttons">
			<span class="submit primary"><input type="submit" name="update" value="{$lang_profile['Update profile']}" /></span>
		</div>
	</form>

END;
    echo '</div>';

    $tpl_temp = forum_trim(ob_get_contents());
    $tpl_main = str_replace('<!-- forum_main -->', $tpl_temp, $tpl_main);
    ob_end_clean();
    // END SUBST - <!-- forum_main -->

    require FORUM_ROOT . 'footer.php';
}
