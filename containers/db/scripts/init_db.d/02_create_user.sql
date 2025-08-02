CREATE TABLE users (
    id SERIAL PRIMARY KEY, -- 기본 키 (자동 증가)
    email VARCHAR(255), -- 이메일 (NULL 허용)
    password_hash VARCHAR(255), -- 비밀번호 해시 (NULL 허용)
    login_type login_type NOT NULL, -- Enum 타입 (NOT NULL)
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP, -- 생성 시간
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP -- 수정 시간
);
