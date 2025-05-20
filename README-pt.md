# chaostool

Esta é uma ferramenta de caos criada por e para Pedro Afonso, adaptada para meus casos de uso específicos.

## Funcionalidades Atuais
- **Consumidor de CPU:** Simula alto uso de CPU alocando múltiplos núcleos por um tempo determinado.
- **Consumidor de RAM:** Simula alto uso de memória alocando uma quantidade específica de RAM por um tempo determinado.
- **Consumidor de Conexões ao Banco:** Abre e mantém várias conexões ao banco de dados para simular cenários de exaustão de recursos.
- **Simulação de Falha no Banco:** Bloqueia temporariamente o acesso a um banco remoto manipulando regras de firewall via SSH.
- **Simulação de Falha em VM:** Simula falha de rede em uma máquina virtual bloqueando todo o tráfego de entrada e saída via firewall por SSH.
- **Deleção Aleatória de Container Docker:** Remove aleatoriamente um container Docker em execução para testes de resiliência.
- **Prune de Containers Docker:** Remove todos os containers Docker de uma vez para cenários de limpeza total.

## Funcionalidades Planejadas
- **Picos de latência aleatórios:** Picos de latência aleatórios em requisições de rede.
- **Interrupção de requisições:** Torna a máquina inacessível por alguns segundos, simulando uma falha de internet.

---

## Instalação

1. **Clone o repositório:**
   ```sh
   git clone <seu-repo-url>
   cd chaostool
   ```
2. **Compile o projeto (requer Rust e Cargo):**
   ```sh
   cargo build --release
   ```
3. **O binário estará disponível em:**
   ```sh
   target/release/chaostool
   ```

---

## Uso

### Consumidor de CPU
Simula alto uso de CPU:
```sh
chaostool cpuhog --cores 2 --seconds 10 [--remove-safety]
```
- `--cores`: Número de núcleos de CPU a consumir
- `--seconds`: Duração do consumo
- `--remove-safety`: (Opcional) Remove limites de segurança (use com cuidado)

### Consumidor de RAM
Simula alto uso de memória:
```sh
chaostool memhog --megabytes 1024 --seconds 10 [--remove-safety]
```
- `--megabytes`: Quantidade de RAM a alocar (em MB)
- `--seconds`: Duração da alocação
- `--remove-safety`: (Opcional) Remove limites de segurança (use com cuidado)

### Consumidor de Conexões ao Banco
Abre várias conexões ao banco de dados:
```sh
chaostool dbfull --users 50 --seconds 10 --dburl <DATABASE_URL> [--remove-safety]
```
- `--users`: Número de conexões a abrir
- `--seconds`: Duração das conexões
- `--dburl`: String de conexão do banco (ex: postgres://usuario:senha@host/db)
- `--remove-safety`: (Opcional) Remove limites de segurança (use com cuidado)

### Simulação de Falha no Banco
Bloqueia temporariamente o acesso ao banco remoto:
```sh
chaostool dbfailure <REMOTE_HOST> --remote_port 5432 --seconds 10
```
- `<REMOTE_HOST>`: Endereço SSH do host remoto (ex: root@seu-servidor-db)
- `--remote_port`: Porta do serviço de banco (ex: 5432 para PostgreSQL)
- `--seconds`: Duração do bloqueio (em segundos)

**Atenção:** Comando extremamente perigoso. Use apenas em ambientes de teste!

### Simulação de Falha em VM
Bloqueia toda a rede de uma VM via firewall:
```sh
chaostool vmfailure <REMOTE_HOST> --seconds 10
```
- `<REMOTE_HOST>`: Endereço SSH da VM
- `--seconds`: Duração do bloqueio de rede

**Atenção:** Comando extremamente perigoso. Use apenas em ambientes de teste!

### Caos em Containers Docker
Simula caos em containers Docker:
```sh
chaostool dockerkill [NOME_CONTAINER]
chaostool dockerkill --is-random
chaostool dockerkill --prune
```
- `[NOME_CONTAINER]`: Nome do container a parar e remover (argumento posicional)
- `--is-random`: Remove um container em execução aleatoriamente
- `--prune`: Remove todos os containers (em execução e parados)

**Atenção:** Estes comandos são destrutivos. Use com cautela e apenas em ambientes de teste!

---

Esta ferramenta está em desenvolvimento ativo e continuará evoluindo com mais funcionalidades de engenharia do caos adaptadas às minhas necessidades. 