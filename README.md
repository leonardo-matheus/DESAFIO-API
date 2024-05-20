
# API PHP

Quick 'n Dirty.


## Instalação

Instale as dependências

```bash
  sudo apt-get update && sudo apt-get upgrade
```

```bash
  sudo apt install php php-cli php-sqlite3
```


    
## Execução

Para fazer o deploy desse projeto rode

```bash
  php -S 127.0.0.1:8000
```


## Documentação da API

#### Insere 10K de Registros

```http
  http://127.0.0.1:8000/api.php/create
```

#### Recuperar registros

```http
  http://127.0.0.1:8000/api.php/retrieve
```

#### Limpa o DB

```http
  http://127.0.0.1:8000/api.php/clear
```
#### Conta os registros

```http
  http://127.0.0.1:8000/api.php/count
```


# API Rust

Qualidade 👌



## Instalação

Instale as dependências

```bash
  sudo apt install curl
```

```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Após a instalação, você precisará adicionar o diretório cargo/bin ao seu PATH. O instalador geralmente sugere que você execute o seguinte comando:

```bash
  source $HOME/.cargo/env
```

```bash
  rustc --version
  cargo --version
```
## Execução

Para fazer o deploy desse projeto rode

```bash
  cargo build
```

```bash
  cargo run
```

## Documentação da API

#### Insere 10K de Registros

```http
  http://127.0.0.1:8080/create
```

#### Recuperar registros

```http
  http://127.0.0.1:8080/retrieve
```

#### Limpa o DB

```http
  http://127.0.0.1:8080/clear
```
#### Conta os registros

```http
  http://127.0.0.1:8080/count
```

