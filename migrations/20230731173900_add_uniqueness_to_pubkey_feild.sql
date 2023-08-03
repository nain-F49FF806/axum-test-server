-- Rename pubkey
ALTER TABLE accounts
    RENAME COLUMN pubkey_mf to auth_pubkey;

ALTER TABLE accounts
    ADD auth_pubkey_sha BINARY(32) GENERATED ALWAYS AS (
        UNHEX(SHA2(auth_pubkey, 256))
    ) STORED NOT NULL UNIQUE;