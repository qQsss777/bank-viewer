FROM postgres:16
COPY ./tools/sql/user.sql /docker-entrypoint-initdb.d/user.sql
COPY ./tools/conf/pg_hba.conf /etc/postgresql/pg_hba.conf
COPY ./tools/conf/postgresql.conf /etc/postgresql/postgresql.conf
