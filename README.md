# Casper Recordkeeping dApp

This contract provides five entry points:

`approve_supervisor:` This is used by the account that sends the deploy to approve a supervisor. The supervisor's public key is passed as an argument.

`approve_watchstander:` This is used by the supervisor to approve a watchstander. The supervisor's and watchstander's public keys are passed as arguments. The function checks if the supervisor is approved before approving the watchstander.

`make_entry:` This is used by the watchstander to make an entry. The supervisor's and watchstander's public keys, date, time, and entry text are passed as arguments. The function checks if both the supervisor and watchstander are approved before making the entry.

`retrieve_entries_by_date:` This is used to retrieve all entries for a specific date. The date is passed as an argument.

`retrieve_entry_by_date_and_time:` This is used to retrieve a specific entry by date and time. The date and time are passed as arguments.

Please note that this is a basic example and does not include all necessary error handling and security checks. You should thoroughly review and test any smart contract code before deploying it to a live network.

## Casper Client CLI Examples

### Approving a Supervisor

```bash
casper-client put-deploy \
--node-address http://<node-address>:7777 \
--chain-name casper \
--secret-key /path/to/secret_key.pem \
--session-hash CONTRACT_HASH \
--session-entry-point approve_supervisor \
--session-arg "supervisor_public_key:public_key='SUPERVISOR_PUBLIC_KEY'"
```

### Approving a Watchstander

```bash
casper-client put-deploy \
--node-address http://<node-address>:7777 \
--chain-name casper \
--secret-key /path/to/secret_key.pem \
--session-hash CONTRACT_HASH \
--session-entry-point approve_watchstander \
--session-arg "supervisor_public_key:public_key='SUPERVISOR_PUBLIC_KEY'" \
--session-arg "watchstander_public_key:public_key='WATCHSTANDER_PUBLIC_KEY'"
```

### Making an Entry

```bash
casper-client put-deploy \
--node-address http://<node-address>:7777 \
--chain-name casper \
--secret-key /path/to/secret_key.pem \
--session-hash CONTRACT_HASH \
--session-entry-point make_entry \
--session-arg "supervisor_public_key:public_key='SUPERVISOR_PUBLIC_KEY'" \
--session-arg "watchstander_public_key:public_key='WATCHSTANDER_PUBLIC_KEY'" \
--session-arg "date:string='2022-01-01'" \
--session-arg "time:string='12:00:00'" \
--session-arg "entry:string='This is a test entry.'"
```

### Retrieving a List of Entries by Date

```bash
casper-client query-state \
--node-address http://<node-address>:7777 \
--state-root-hash <state-root-hash> \
--key <dictionary-uref> \
--query-path <date>
```

### Retrieving a Specific Entry

```bash
casper-client query-state \
--node-address http://<node-address>:7777 \
--state-root-hash <state-root-hash> \
--key <dictionary-uref> \
--query-path "<date>/<time>"
```