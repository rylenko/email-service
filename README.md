<h1 align="center">Welcome to Email-Service 🌿</h1>
Hidden decentralized email service with E2E encryption and F2F support. Version: 1.7.1

<h1 align="center">Specifications</h1>

**1.** Client protocol: HTTPS.

**2.** Intermediate protocol (When interacting with a node): TCP (with proxy support).

**3.** End to end encryption.

**4.** Friend to friend support.

**5.** Encryption: RSA-PKCS1-OAEP, AES-GCM-256.

**6.** Signing: RSA-PKCS1-PSS.

**7.** Hashing: SHA-256.

<h1 align="center">Todo</h1>

**-** Remove package exchange recursion.

**-** Achieve user from request.

**-** Struct for package data like `(usize, [u8; 32])`.

**-** Capacity by size. If full, then remove old emails.

**-** Delete sent emails.

**-** Change from `openssl` to another library.

**-** Forms whose data is extracted through `client::app::multipart::extract_multipart` cannot have numbers in the fields where the string is expected. For example, when creating an email, we cannot specify a number in the title or text.

**-** If you remove the user in any way without leaving the account, for example by removing the docker containers, it will show that we are still authenticated, but we cannot modify or view anything other than the username.

**-** When loading files for a while, the memory doubles (because one copy of the file is in the form object, and the other in the base-64 email data structure). It would be nice to fix this, otherwise with large files and `common::consts::PACKAGE_BUFFER_SIZE` it can be a nuisance.

**-** Server-side actix-session (Redis).

**-** Cache database calls (Redis).

**-** Make less bloated.

**-** Tests.

**-** https://bestmotherfucking.website/.

**-** Return error code to user. Log traceback to file: "error_code: traceback".

**-** Try & improve in production.

**-** GUI or TUI.

<h1 align="center">Certificates</h1>

By default, self-signed certificates are present in the **docker/client-nginx/certs** folder. If you are going to use these, replace them.

<h1 align="center">Installation</h1>

**1.** Clone this repository how you like it.

**2.** Run tests.
```
$ cd email-service/
$ cargo test --workspace
$ cd ../
```

**3.1.** Create the required .env file with the following options **(email-service/client/.env)**.
```
POSTGRES_USER=user
POSTGRES_PASSWORD=password
POSTGRES_DB=db
DATABASE_URL=postgresql://${POSTGRES_USER}:${POSTGRES_PASSWORD}@client-db:5432/${POSTGRES_DB}
```

**3.2.** You can also enable debugging on the client by adding the following **(email-service/client/.env)**:
```
DEBUG=1
```

**4.1.** Run the client:
```
$ ./run.py client
```

**4.2.** Wait for everything to load, then close the running client.

**5.** Port configuration. Here you can specify which port your client will run on. Example **(ports.json)**:
```
{
	...
	"client": 8000
}
```

**6.** In the client config you can specify the dark theme, the SOCKS5 `proxy` from which all requests to the node will be sent, as well as a `secret_key` to set the cookie. Example **(email-service/client/config.json)**:
```
{
	"dark_theme": true,
	"proxy": "123.456.78.90:1234",
	"secret_key": "super-secret-key-123"
}
```

**7.** Launch the client:
```
$ ./run.py client
```

**8.** Open in your browser <a href="https://127.0.0.1:9999">https://127.0.0.1:9999</a>. Make sure that you specify the same port as in the config. Log in to your account on <a href="https://127.0.0.1:9999/login/">this page</a>.

**9.** Then go to the <a href="https://127.0.0.1:9999/nodes/add/">add node page</a> and add a node you know. If you don't have one, go to the next section on how to deploy it.

**10.** Then add friend on <a href="https://127.0.0.1:999/friends/add/">this page</a>.

**11.** Preparation is complete, you can send emails on <a href="https://127.0.0.1:9999/emails/send/">this page</a>.

<h1 align="center">Deploying and interacting with a node</h1>

**1.1.** Create the required .env file with the following options **(email-service/node/.env)**.
```
POSTGRES_USER=user
POSTGRES_PASSWORD=password
POSTGRES_DB=db
DATABASE_URL=postgresql://${POSTGRES_USER}:${POSTGRES_PASSWORD}@node-db:5432/${POSTGRES_DB}
```

**1.2.** You can also enable debugging on the node by adding the following **(email-service/node/.env)**:
```
DEBUG=1
```

**2.1.** Run the node:
```
$ ./run.py node
```

**2.2** Wait for everything to load, then close the running node.

**3.** Port configuration. Here you can specify which port your node will run on. Example **(ports.json)**:
```
{
	"node:" 8888,
	...
}
```

**4.** In the node configuration, you can specify a password, as well as other nodes to which, for example, our node will forward received emails. Example **(email-service/node/config.json)**:
```
{
	"password": "super-password-123",
	"other_nodes": [
		{"address": "127.456.78.90:1234", "password": null}
	]
}
```

**5.** Launch the node:
```
$ ./run.py node
```

**6.** Your node is now deployed. You can add it on the client side in `ipv4:port` format. Where `ipv4` is the private IP, something like 192.168.x.xx (You can look it up with `ip -4 addr` and port is the port you specified in **ports.json**. Also don't forget to specify the password.
