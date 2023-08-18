-- Add migration script here

CREATE TABLE IF NOT EXISTS accounts (
    -- metadata (total accounts registered until this account's creation)
    seq_num SERIAL,
    -- sequential time ordered uuid
    -- https://dev.mysql.com/blog-archive/mysql-8-0-uuid-support/
    account BINARY(16) DEFAULT (UUID_TO_BIN(UUID(),true)) NOT NULL UNIQUE,
    -- for display purpose
    account_name CHAR(36) GENERATED ALWAYS AS (BIN_TO_UUID(account)) VIRTUAL,
    -- mediator facing pubkey
    auth_pubkey VARCHAR(64) NOT NULL UNIQUE,
    PRIMARY KEY(account)
);


CREATE TABLE IF NOT EXISTS recipients (
    seq_num SERIAL,
    account BINARY(16) NOT NULL,
    recipient_key VARCHAR(64) NOT NULL,
    PRIMARY KEY (account, recipient_key),
    FOREIGN KEY (account) REFERENCES accounts(account)
);


CREATE TABLE IF NOT EXISTS messages (
    seq_num SERIAL,
    account BINARY(16) NOT NULL,
    recipient_key VARCHAR(64) NOT NULL,
    message_id BINARY(16) DEFAULT (UUID_TO_BIN(UUID(),true)) NOT NULL UNIQUE,
    -- 16MiB limit (medium blob)
    message_data MEDIUMBLOB NOT NULL,
    PRIMARY KEY (account, recipient_key, message_id),
    FOREIGN KEY (account, recipient_key) REFERENCES recipients(account, recipient_key)
);


