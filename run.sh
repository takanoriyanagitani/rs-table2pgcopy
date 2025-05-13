#!/bin/sh

mkdir -p sample.d

which usql || exec sh -c 'echo usql missing.; exit 1'

export ENV_TABLE_NAME=tab3

export PGHOST=127.0.0.1
export PGUSER=postgres
export PGDATABASE=postgres
export PGPASSWORD="${PGPASSWORD}"

usql "pg://${PGUSER}@${PGHOST}" -c "
	CREATE TABLE IF NOT EXISTS tab3(
		id BIGSERIAL PRIMARY KEY,
		key TEXT NOT NULL,
		val TEXT NOT NULL
	)
"

usql "pg://${PGUSER}@${PGHOST}" -c "
	INSERT INTO tab3(key, val)
	SELECT 'helo', 'wrld'
	WHERE 1 <> COALESCE(
		(SELECT id FROM tab3 WHERE key='helo'),
		0
	)
"

./rs-table2pgcopy |
	xxd
