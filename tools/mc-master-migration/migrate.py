#!/usr/bin/env python3
import shutil
import uuid
import mysql.connector

DB_HOST = 'localhost'
DB_USER = 'user'
DB_USER_PASSWORD = 'password'
DB_NAME_OLD = 'master'
DB_NAME_NEW = 'ttyhmaster'
MIGRATE_STATS = True

MIGRATE_SKINS = True
SKINS_DIR_OLD = 'path/to/skins/old'
SKINS_DIR_NEW = 'path/to/skins/new'


def migrate_players(old_conn, new_conn):
    old_cur = old_conn.cursor()
    new_cur = new_conn.cursor()

    old_cur.execute(
        """
        SELECT
            player, clientToken, password, salt, isMojang, skin, skin_model, accessToken
        FROM 
            players
        """
    )
    players = old_cur.fetchall()

    for (name, player_id, pwd_hash, salt, is_mojang, skin, skin_model, access_token) in players:
        access_token = access_token if access_token is not None else str(uuid.uuid4())
        is_slim = skin_model == 'slim'

        query = """
        INSERT INTO players
            (player_name, player_id, password_hash, salt, is_mojang, is_slim_model, access_token)
        VALUES
            (%s, %s, %s, %s, %s, %s, %s)
        """
        values = (name, player_id, pwd_hash, salt, is_mojang, is_slim, access_token)
        new_cur.execute(query, values)
        new_conn.commit()

        if MIGRATE_SKINS and skin is not None:
            shutil.copy(f'{SKINS_DIR_OLD}/{skin}', f'{SKINS_DIR_NEW}/{player_id}')


def migrate_stats(old_conn, new_conn):
    old_cur = old_conn.cursor()
    new_cur = new_conn.cursor()

    old_cur.execute(
        """
        SELECT
            player, uuid, ticket, launcher_ver, os, os_version, os_arch, created_at
        FROM 
            ids
        """
    )
    entries = old_cur.fetchall()

    for (name, hw_id, sw_id, version, os, os_version, os_arch, created_at) in entries:
        if None in [name, hw_id, sw_id, version, os, os_version, os_arch, created_at]:
            continue

        query = """
        INSERT INTO stats
            (player_name, launcher_ver, os, os_version, os_word_size, install_uuid, machine_uuid, created_at)
        VALUES
            (%s, %s, %s, %s, %s, %s, %s, %s)
        """
        values = (name, version, os, os_version, os_arch, sw_id, hw_id, created_at)
        new_cur.execute(query, values)
        new_conn.commit()


def main():
    old_conn = mysql.connector.connect(host=DB_HOST, user=DB_USER, password=DB_USER_PASSWORD,
                                       database=DB_NAME_OLD,
                                       charset='utf8mb4', collation='utf8mb4_general_ci')

    new_conn = mysql.connector.connect(host=DB_HOST, user=DB_USER, password=DB_USER_PASSWORD,
                                       database=DB_NAME_NEW,
                                       charset='utf8mb4', collation='utf8mb4_general_ci')

    migrate_players(old_conn, new_conn)
    if MIGRATE_STATS:
        migrate_stats(old_conn, new_conn)


if __name__ == '__main__':
    main()
