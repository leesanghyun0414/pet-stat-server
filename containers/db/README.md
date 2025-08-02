# POSTGRES_PASSWORD
This environment variable is required for you to use the PostgreSQL image. It must not be empty or undefined. This environment variable sets the superuser password for PostgreSQL. The default superuser is defined by the POSTGRES_USER environment variable.

Note 1: The PostgreSQL image sets up trust authentication locally so you may notice a password is not required when connecting from localhost (inside the same container). However, a password will be required if connecting from a different host/container.

Note 2: This variable defines the superuser password in the PostgreSQL instance, as set by the initdb script during initial container startup. It has no effect on the PGPASSWORD environment variable that may be used by the psql client at runtime, as described at https://www.postgresql.org/docs/14/libpq-envars.html⁠. PGPASSWORD, if used, will be specified as a separate environment variable.

# POSTGRES_USER
This optional environment variable is used in conjunction with POSTGRES_PASSWORD to set a user and its password. This variable will create the specified user with superuser power and a database with the same name. If it is not specified, then the default user of postgres will be used.

Be aware that if this parameter is specified, PostgreSQL will still show The files belonging to this database system will be owned by user "postgres" during initialization. This refers to the Linux system user (from /etc/passwd in the image) that the postgres daemon runs as, and as such is unrelated to the POSTGRES_USER option. See the section titled "Arbitrary --user Notes" for more details.

# POSTGRES_DB
This optional environment variable can be used to define a different name for the default database that is created when the image is first started. If it is not specified, then the value of POSTGRES_USER will be used.

# POSTGRES_INITDB_ARGS
This optional environment variable can be used to send arguments to postgres initdb. The value is a space separated string of arguments as postgres initdb would expect them. This is useful for adding functionality like data page checksums: -e POSTGRES_INITDB_ARGS="--data-checksums".

# POSTGRES_INITDB_WALDIR
This optional environment variable can be used to define another location for the Postgres transaction log. By default the transaction log is stored in a subdirectory of the main Postgres data folder (PGDATA). Sometimes it can be desireable to store the transaction log in a different directory which may be backed by storage with different performance or reliability characteristics.

Note: on PostgreSQL 9.x, this variable is POSTGRES_INITDB_XLOGDIR (reflecting the changed name of the --xlogdir flag to --waldir in PostgreSQL 10+⁠).


# POSTGRES_HOST_AUTH_METHOD
This optional variable can be used to control the auth-method for host connections for all databases, all users, and all addresses. If unspecified then scram-sha-256 password authentication⁠ is used (in 14+; md5 in older releases). On an uninitialized database, this will populate pg_hba.conf via this approximate line:

```bash
echo "host all all all $POSTGRES_HOST_AUTH_METHOD" >> pg_hba.conf
```



# PostgreSQL Docker 컨테이너 초기화 스크립트 및 데이터베이스 설정

## 초기화 스크립트
PostgreSQL Docker 이미지를 기반으로 추가 초기화를 수행하려면, 아래 지침을 따르세요.

### 초기화 스크립트 경로
`/docker-entrypoint-initdb.d` 디렉토리에 다음과 같은 스크립트를 추가하세요:
- `*.sql`
- `*.sql.gz`
- `*.sh`

**주의:** 
- 해당 디렉토리 안의 스크립트는 데이터 디렉토리가 비어있는 경우에만 실행됩니다. 이미 초기화된 데이터 디렉토리에서는 스크립트가 실행되지 않습니다.
- 스크립트가 실패하면 컨테이너가 종료되며, 데이터 디렉토리가 초기화된 상태로 남아 이후 스크립트는 실행되지 않습니다.

### 예제: 사용자 및 데이터베이스 추가
`/docker-entrypoint-initdb.d/init-user-db.sh` 파일을 생성하고 아래 내용을 추가합니다.

```bash
#!/bin/bash
set -e

psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" <<-EOSQL
	CREATE USER docker;
	CREATE DATABASE docker;
	GRANT ALL PRIVILEGES ON DATABASE docker TO docker;
EOSQL
```

스크립트 실행 순서
스크립트는 현재 로케일(기본값: en_US.utf8)에 따라 정렬된 이름 순서로 실행됩니다.

실행 권장사항
*.sh 스크립트에서 psql 명령어는 항상 POSTGRES_USER로 실행하세요. 예: --username "$POSTGRES_USER"
초기화 스크립트는 PostgreSQL 컨테이너 내부의 postgres 사용자로 실행됩니다.
데이터베이스 설정
설정 방법
PostgreSQL 서버 설정은 다음 방법으로 가능합니다:

커스텀 설정 파일 사용 PostgreSQL 기본 샘플 파일을 가져와 수정 후 컨테이너에서 사용합니다.

```sh
# 기본 설정 파일 복사
docker run -i --rm postgres cat /usr/share/postgresql/postgresql.conf.sample > my-postgres.conf

# 커스텀 설정 파일을 사용하여 컨테이너 실행
docker run -d --name some-postgres \
    -v "$PWD/my-postgres.conf":/etc/postgresql/postgresql.conf \
    -e POSTGRES_PASSWORD=mysecretpassword \
    postgres -c 'config_file=/etc/postgresql/postgresql.conf'
```

참고:

다른 컨테이너에서 PostgreSQL에 접근하려면 listen_addresses = '*'로 설정해야 합니다.
docker run 명령으로 직접 옵션 설정 .conf 파일에 설정 가능한 모든 옵션을 -c를 통해 설정할 수 있습니다.

```bash
docker run -d --name some-postgres \
    -e POSTGRES_PASSWORD=mysecretpassword \
    postgres -c shared_buffers=256MB -c max_connections=200
```
