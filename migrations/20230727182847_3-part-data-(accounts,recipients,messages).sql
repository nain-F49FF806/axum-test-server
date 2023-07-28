-- Add migration script here

CREATE TABLE IF NOT EXISTS accounts (
    id SERIAL,
    -- sequential time ordered uuid
    -- https://dev.mysql.com/blog-archive/mysql-8-0-uuid-support/
    account BINARY(16) DEFAULT (UUID_TO_BIN(UUID(),true)) NOT NULL UNIQUE,
    account_name VARCHAR(36) GENERATED ALWAYS AS (BIN_TO_UUID(account)) VIRTUAL,
    -- mediator facing pubkey
    pubkey_mf TEXT NOT NULL,
    -- live mode active or not
    live_delivery BOOLEAN NOT NULL DEFAULT FALSE,
    PRIMARY KEY(account)
);

CREATE TABLE IF NOT EXISTS recipients (
    id SERIAL,
    account BINARY(16) NOT NULL,
    -- need not be unique (what if two accounts somehow register for same key?)
    -- the illegitimate won't be able to decrypt, even if they might be able to register reciept
    recipient_key TEXT NOT NULL,
    -- Consider using combination (recipient_key, account) as the primary key
    -- Done this way, using sha2 because recipient_key is TEXT (not directly indexable)
    -- recipient_key is text because key size, format may be variable
    recipient BINARY(32) GENERATED ALWAYS AS (
        UNHEX(
            SHA2(
                CONCAT((BIN_TO_UUID(account)), "+", recipient_key), 256
            )
        )
    ) STORED,
    PRIMARY KEY (recipient),
    FOREIGN KEY (account) REFERENCES accounts(account)
);

-- -- To be created programmatically on account creation
-- CREATE TABLE IF NOT EXISTS `messages_for_{account_name}` (
--     id SERIAL PRIMARY KEY,
--     -- need not be unique (what if two accounts somehow register for same key?)
--     -- the illegitimate won't be able to decrypt, even if they might be able to register reciept
--     recipient_key TEXT NOT NULL,
--     -- 16MiB limit (medium blob)
--     message_data MEDIUMBLOB NOT NULL,
-- );


CREATE TABLE IF NOT EXISTS `messages` (
    id SERIAL,
    recipient BINARY(32) NOT NULL,
    message_id BINARY(16) DEFAULT (UUID_TO_BIN(UUID(),true)) NOT NULL UNIQUE,
    -- 16MiB limit (medium blob)
    message_data MEDIUMBLOB NOT NULL,
    PRIMARY KEY (message_id),
    FOREIGN KEY (recipient) REFERENCES recipients(recipient)
);

