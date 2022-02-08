# introduction
This is rust implementation of  [go-chat](https://github.com/kone-net/go-chat).

This project is to use rust to do what go can do.

They share the same database and frontend.

# comparison




|                 | go-chat                        | rust-chat      |
| --------------- | ------------------------------ | -------------- |
| backend         | gin                            | axum           |
| frontend        | react                          | react          |
| database        | mysql                          | mysql          |
| orm/sql-package | gorm                           | sqlx           |
| log             | zap                            | tracing        |
| config          | viper                          | dotenv         |
| protobuf        | github.com/gogo/protobuf/proto | prost          |
| coroutine       | go routine                     | tokio task     |
| channel         | go channel                     | tokio channel  |
| websocket       | github.com/gorilla/websocket   | axum websocket |

# dependency

* rust
* mysql

# steps to build

## backend

1. get project

   ```shell
   git clone git@github.com:21pages/rust-chat.git
   cd rust-chat
   ```

2. create mysql database with `chat.sql`

3. modify `.env`

4. sqlx preparation

   ```shell
   cargo install sqlx-cli
   cargo sqlx prepare
   ```

5. build && run

   ```shell
   cargo run
   ```

## frontend

1. get project

    ```shell
    git clone git@github.com:kone-net/go-chat-web.git
    cd go-chat-web
    ```

2. modify `IP_PORT`in`src/common/param/Params.jsx`

3. prepare npm environment

   ```
   npm install
   ```

4. run project

   ```shell
   npm start
   ```

5. visit frontend entry `http://127.0.0.1:3000`

# schedule

- [x] | axum router, cors
- [x] | sqlx, model map
- [x] | user register, login, friends, group join
- [x] | message
- [x] | appstate, channel, websocket
- [x] | file upload & download
- [ ] | kafka
