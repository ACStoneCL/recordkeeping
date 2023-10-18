# Casper Recordkeeping dApp

This contract provides five entry points:

`approve_supervisor:` This is used by the account that sends the deploy to approve a supervisor. The supervisor's public key is passed as an argument.

`approve_watchstander:` This is used by the supervisor to approve a watchstander. The supervisor's and watchstander's public keys are passed as arguments. The function checks if the supervisor is approved before approving the watchstander.

`make_entry:` This is used by the watchstander to make an entry. The supervisor's and watchstander's public keys, date, time, and entry text are passed as arguments. The function checks if both the supervisor and watchstander are approved before making the entry.

`retrieve_entries_by_date:` This is used to retrieve all entries for a specific date. The date is passed as an argument.

`retrieve_entry_by_date_and_time:` This is used to retrieve a specific entry by date and time. The date and time are passed as arguments.

Please note that this is a basic example and does not include all necessary error handling and security checks. You should thoroughly review and test any smart contract code before deploying it to a live network.
