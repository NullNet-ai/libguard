## liblogging

Logging library for Nullnet.

Use this library simply by calling the `Logger::init` method with the desired configuration,
and then invoking the `log` macros.

It handles logs to **console**, **syslog**, and **`PostgreSQL`**.<br>
Each of these loggers can be enabled or disabled independently.

### Log levels

You can configure the log level by setting the `LOG_LEVEL` environment variable. The possible log levels are:
- `OFF`
- `ERROR`
- `WARN`
- `INFO`
- `DEBUG`
- `TRACE`

If you don't set the `LOG_LEVEL` environment variable, `TRACE` will be used by default (the most verbose level).

### `PostgreSQL` logger

The `PostgreSQL` logger will log messages to a `PostgreSQL` database.<br>
You can configure the connection by setting the following environment variables:
- `POSTGRES_HOST` (default: `localhost`)
- `POSTGRES_PORT` (default: `5432`)
- `POSTGRES_USER` (default: `postgres`)
- `POSTGRES_PASSWORD` (default: `postgres`)
- `POSTGRES_DB_NAME` (default: `postgres`)
- `POSTGRES_TABLE_NAME` (default: `logs`)

### Allowed targets

By default, only logs from targets in the form `nullnet*`, `appguard*`, and `wallguard*` will be emitted.

To allow additional targets, set them in the `LoggerConfig` passed to the `Logger::init` method
(e.g., specifying "serde" will emit logs for all targets whose name is in the form `serde*`).
